use std::collections::HashSet;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cookies::TOKEN_COOKIE,
    db::Database,
    email::MailClient,
    forms::users::{InvitedMultipleUsersForm, LoginForm, NewUserForm, Password},
    models::users::{NewUser, PublicUser, User},
    schema::users,
};
use diesel::{
    result::{DatabaseErrorKind, Error as DieselError},
    RunQueryDsl,
};
use rocket::{form::Form, http::CookieJar, serde::json::Json};
use uuid::Uuid;

use super::database;

const MAX_SIMILAR_USERNAMES: usize = 100;

#[post("/login", data = "<credentials>")]
pub async fn login_by_form(
    credentials: Form<LoginForm<'_>>,
    db: Database,
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
    db: Database,
) -> Result<Success<Null>, Error<Null>> {
    // Create a vector of Users and a HashSet of base usernames from the form
    let (mut new_users, base_usernames) = form
        .get_users_and_base_usernames()
        .map_err(ApiResponse::internal_server_error)?;

    // Collect duplicate usernames from the database
    let mut existing_usernames = database::get_username_duplicates(&db, &base_usernames).await?;

    // Update the usernames of the new users to avoid unique constraint violations
    update_usernames(&mut new_users, &mut existing_usernames).map_err(ApiResponse::bad_request)?;

    // Insert the new users into the database in a single transaction
    // This transaction will be rolled back on failure
    let inserted_count = database::create_transaction_bulk_invitation(&new_users, &db).await?;

    // Send an invitation email to the new users
    let inviter = guard.get_user();
    let mail_client = MailClient::no_reply();
    for user in new_users {
        let recipient = PublicUser::from(&user);

        mail_client
            .send_invitation(&inviter, &recipient, form.space)
            .map_err(|e| ApiResponse::internal_server_error(format!("Coudn't send email: {e}")))?;
    }

    // Return success response
    Ok(ApiResponse::success(
        format!("{inserted_count} users invited"),
        None,
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
    db: Database,
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
    let inserted_user: User = db
        .run(move |conn| {
            diesel::insert_into(users::table)
                .values(&new_user)
                .get_result(conn)
        })
        .await
        .map_err(|e| match e {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                ApiResponse::conflict("User already exists".to_string(), e.to_string())
            }
            _ => ApiResponse::internal_server_error(format!("Error creating user: {}", e)),
        })?;

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
pub async fn inject_user(user: Json<User>, db: Database) -> String {
    let mut new_user = user.into_inner(); // Extract user data from Json
    let username = new_user.username.clone();
    new_user.id = Uuid::new_v4(); // Generate a new UUID

    // Use Diesel to insert the new user
    let result = db
        .run(move |c| {
            diesel::insert_into(users::table)
                .values(&new_user) // Clone new_user into the closure
                .execute(c) // Pass the connection
        })
        .await;

    match result {
        Ok(_) => format!("User {username} created"),
        Err(e) => format!("Error creating user: {e}"), // Print error details
    }
}

fn update_usernames(
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
