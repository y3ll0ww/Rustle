use std::collections::HashSet;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    cookies::TOKEN_COOKIE,
    database::{users as database, Db},
    email::MailClient,
    forms::users::{InvitedMultipleUsersForm, LoginForm, NewUserForm, Password},
    models::users::{NewUser, PublicUser, User},
};
use rocket::{form::Form, http::CookieJar, serde::json::Json, State};
use uuid::Uuid;

const MAX_SIMILAR_USERNAMES: usize = 100;

#[post("/login", data = "<credentials>")]
pub async fn login_by_form(
    credentials: Form<LoginForm<'_>>,
    db: Db,
    cookies: &CookieJar<'_>,
) -> Result<Success<Null>, Error<Null>> {
    // Get the user from the database
    let user = database::get_user_by_username(&db, credentials.username).await?;

    // Validate if the given password is correct
    if !Password::verify_password(credentials.password, &user.password).map_err(|e| {
        ApiResponse::internal_server_error(format!("Password verification failed: {}", e))
    })? {
        return Err(ApiResponse::bad_request("Invalid password".to_string()));
    };

    // Add the user to the JWT guard
    JwtGuard::secure(&user, cookies)
        .await
        .map_err(ApiResponse::internal_server_error)?;

    // Return the token
    Ok(ApiResponse::success("Login successful".to_string(), None))
}

#[post("/logout")]
pub fn logout(_guard: JwtGuard, cookies: &CookieJar<'_>) -> Success<String> {
    cookies.remove_private(TOKEN_COOKIE);

    ApiResponse::success(
        "Logout successful - token and user info removed".to_string(),
        None,
    )
}

#[post("/invite", data = "<form>")]
pub async fn invite_new_users_by_form(
    guard: JwtGuard,
    form: Form<InvitedMultipleUsersForm<'_>>,
    db: Db,
    redis: &State<RedisMutex>,
) -> Result<Success<Vec<String>>, Error<Null>> {
    // Create a vector of Users and a HashSet of base usernames from the form
    let (mut new_users, base_usernames) = form
        .get_users_and_base_usernames()
        .map_err(ApiResponse::internal_server_error)?;

    // Collect duplicate usernames from the database
    let mut existing_usernames = database::get_username_duplicates(&db, &base_usernames).await?;

    // Update the usernames of the new users to avoid unique constraint violations
    assign_unique_usernames(&mut new_users, &mut existing_usernames)
        .map_err(ApiResponse::bad_request)?;

    // Insert the new users into the database in a single transaction
    let inserted_count = database::create_transaction_bulk_invitation(&new_users, &db).await?;

    // Declare a vector to keep the tokens
    let mut tokens = Vec::new();

    // Loop through the collection of new users
    for user in new_users {
        // Create a random token with a length of 64 characters
        let token = cache::create_random_token(64);

        // Save the token for the response
        tokens.push(token.clone());

        // Add the token to the redis cache; containing the user ID
        cache::users::add_invite_token(redis, &token, user.id).await?;

        let inviter = guard.get_user();
        let recipient = PublicUser::from(&user);
        let team_name = form.space.to_string();

        // Send an invitation email to the new users, containing the token
        tokio::spawn(async move {
            let _ =
                MailClient::no_reply().send_invitation(&inviter, &recipient, &team_name, &token);
        });
    }

    // Return success response
    Ok(ApiResponse::success(
        format!("{inserted_count} users invited"),
        Some(tokens),
    ))
}

/// This function allows for the creation of a new [`User`] by using a form.
///
/// **Route**: `./form`
///
/// ### Parameters
/// * `db`: Instance of the [`Database`] connection.
/// * `form`: A [`NewUserForm`] for creating a [`User`].
///
/// ### Returns
/// * `Ok(Success<InsertedUser>)`: When `Ok`, it returns [`Success`] with the [`InsertedUser`].
/// * `Err(Error<String>)`: When `Err`, it returns an [`Error`] with [`Null`].
#[post("/register", data = "<form>")]
pub async fn create_new_user_by_form(
    form: Form<NewUserForm<'_>>,
    db: Db,
    cookies: &CookieJar<'_>,
) -> Result<Success<Null>, Error<Null>> {
    // Hash the provided password
    let password_hash = form
        .password
        .hash_password()
        .map_err(|e| ApiResponse::internal_server_error(format!("Couldn't hash password: {e}")))?;

    // Create a new User
    let new_user = NewUser {
        username: form.username.to_string(),
        display_name: form.username.to_string(),
        email: form.email.to_string(),
        password: password_hash,
    };

    // Add the new User to the database
    let inserted_user = database::create_new_user(&db, new_user).await?;

    // Add the user to the JWT guard
    JwtGuard::secure(&inserted_user, cookies)
        .await
        .map_err(ApiResponse::internal_server_error)?;

    // Return success response
    Ok(ApiResponse::success(
        format!("User '{}' created succesfully", inserted_user.username),
        None,
    ))
}

#[post("/create", format = "json", data = "<user>")]
pub async fn inject_user(user: Json<User>, db: Db) -> String {
    let mut new_user = user.into_inner(); // Extract user data from Json
    let username = new_user.username.clone();
    new_user.id = Uuid::new_v4(); // Generate a new UUID

    // Use Diesel to insert the new user
    match database::inject_user(&db, new_user).await {
        Ok(_) => format!("User {username} created"),
        Err(e) => format!("Error creating user: {e}"), // Print error details
    }
}

fn assign_unique_usernames(
    new_users: &mut [User],
    existing_usernames: &mut HashSet<String>,
) -> Result<(), String> {
    // Loop through the new users and check if their usernames are already taken
    for user in new_users.iter_mut() {
        let mut suffix = 1;
        let mut assigned_username = user.username.clone();

        // If the username is already taken, append a suffix
        while existing_usernames.contains(&assigned_username) {
            assigned_username = format!("{}_{}", user.username, suffix);
            suffix += 1;

            // If the suffix is greater than the maximum, return an error
            if suffix > MAX_SIMILAR_USERNAMES {
                return Err(format!("Too many usernames containing '{}'", user.username));
            }
        }

        // Add the unique username to the existing usernames set
        existing_usernames.insert(assigned_username.clone());

        // Update the username with the unique username
        user.username = assigned_username;
    }

    Ok(())
}
