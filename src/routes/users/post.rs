use std::collections::HashSet;

use crate::{email::MailClient, forms::users::InvitedMultipleUsersForm};
use diesel::RunQueryDsl;

use super::*;

const MAX_SIMILAR_USERNAMES: usize = 100;

pub async fn login_by_form(
    credentials: Form<LoginForm<'_>>,
    db: Database,
    cookies: &CookieJar<'_>,
) -> Result<Success<Null>, Error<Null>> {
    // Get the user from the database
    let user = match &get_user_from_db(db, credentials.username).await?.data {
        Some(user) => user.clone(),
        None => {
            return Err(ApiResponse::internal_server_error(
                "No user data".to_string(),
            ));
        }
    };

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

pub fn logout(_guard: JwtGuard, cookies: &CookieJar<'_>) -> Success<String> {
    cookies.remove_private(TOKEN_COOKIE);

    ApiResponse::success(
        "Logout successful - token and user info removed".to_string(),
        None,
    )
}

pub async fn invite_new_users_by_form(
    guard: JwtGuard,
    form: Form<InvitedMultipleUsersForm<'_>>,
    db: Database,
) -> Result<Success<Null>, Error<Null>> {
    // 1) Extract all the users from the form
    //    a) Create display_name and username based on first and last name
    //    b) Create a hashed password based on a generated UUID
    // 2) Get a list of all the users in the database that have similar usernames as the ones in
    //    the newly created list of users
    //    a) Where users match any of the following names: "full_username"
    //    b) Take the current usernames into consideration
    // 3) Assign a username with a counter "_1",  "_2", "_3" etc.
    // 4) Insert all the users in one transaction batch; rollback at error

    let mut new_users = Vec::new();
    let mut base_usernames = HashSet::new();

    for user in &form.users {
        // Define the username and the display name
        let display_name = format!("{} {}", user.first_name, user.last_name);
        let username = display_name.to_lowercase().replace(' ', "_");

        base_usernames.insert(username.clone());

        // Generate a hashed password
        let password = Password::generate(None).map_err(|e| {
            ApiResponse::internal_server_error(format!("Coudn't hash password: {e}"))
        })?;

        // Add a new user to be processed
        new_users.push(User::new(
            username,
            Some(display_name),
            user.email.to_string(),
            password,
        ));
    }
    ///////////////////////////////////////////////////////////////////////////////////////////////
    //.filter(sql("username ~ '^john_doe(_[0-9]+)?$'"))

    // Get all existing similar usernames from the database
    let mut existing_usernames: HashSet<String> = db
        .run({
            // Pattern for matching usernames; exact and numbered variants:
            // ^(john_doe|jane_doe|john_smith|jane_smith)(_[0-9]+)?$
            let regex_pattern = format!(
                "^({})(_[0-9]+)?$",
                base_usernames
                    .iter()
                    .map(|name| regex::escape(name))
                    .collect::<Vec<_>>()
                    .join("|")
            );

            // Get all existing usernames from the database using the regex pattern, then collect
            // their usernames into a HashSet
            move |conn| {
                diesel::sql_query("SELECT * FROM users WHERE username ~ $1")
                    .bind::<diesel::sql_types::Text, _>(&regex_pattern)
                    .load::<User>(conn)
                    .map(|users| users.into_iter().map(|u| u.username).collect())
            }
        })
        .await
        .map_err(|e| ApiResponse::internal_server_error(e.to_string()))?;

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
                return Err(ApiResponse::bad_request(format!(
                    "Too many usernames containing '{}'",
                    user.username,
                )));
            }
        }

        // Add the successful candidate to the used names
        existing_usernames.insert(assigned_username.clone());

        // Update the username of the to be added user
        user.username = assigned_username;
    }

    // Insert into database with a single transaction
    let inserted_count = db
        .run({
            // Clone the new_users vector to move into the closure
            let insert_users = new_users.clone();

            // Move the database connection into the closure
            move |conn| {
                // Insert all users in one transaction; if any error occurs, rollback
                conn.build_transaction().read_write().run(|conn| {
                    diesel::insert_into(users::dsl::users)
                        .values(&insert_users)
                        .execute(conn)
                })
            }
        })
        .await
        .map_err(|e| ApiResponse::internal_server_error(e.to_string()))?;

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
