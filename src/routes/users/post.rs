use diesel::sql_types::Jsonb;

use crate::{email::MailClient, forms::users::InvitedMultipleUsersForm};

use super::*;

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

pub async fn invite_new_user_by_form(
    guard: JwtGuard,
    form: Form<InvitedMultipleUsersForm<'_>>,
    db: Database,
) -> Result<Success<Null>, Error<Null>> {
    let mut new_users = Vec::new();

    for user in &form.users {
        // Define the username and the display name
        let display_name = format!("{} {}", user.first_name, user.last_name);
        let username = display_name.to_lowercase().replace(' ', "_");

        // Generate a hashed password
        let password = Password::generate().map_err(|e| {
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

    // Insert all new users in a single transaction
    let users_to_database = new_users.clone();
    //let entries = db
    //    .run(move |conn| {
    //        diesel::insert_into(users::table)
    //            .values(users_to_database)
    //            .execute(conn)
    //            .map_err(|e| e.to_string())
    //    })
    //    .await
    //    .map_err(ApiResponse::internal_server_error)?;
    let entries = db
        .run(move |conn| {
            // Create a raw SQL query to call your custom PostgreSQL function
            let users_to_insert = serde_json::to_value(&users_to_database)
                .map_err(|e| format!("Serialization error: {}", e))?;

            // Convert the users to a format that PostgreSQL can handle as an array (this depends on your data format)
            let query = r#"
            SELECT insert_users_if_unique($1::users[]);
        "#;

        diesel::sql_query(query)
                .bind::<Jsonb, _>(users_to_insert) // Bind the serialized JSON data
                .execute(conn)
                .map_err(|e| e.to_string())

         //   diesel::sql_query(query)
         //       .bind::<diesel::sql_types::Text, _>(users_to_insert)
         //       .execute(conn)
         //       .map_err(|e| e.to_string())
        })
        .await
        .map_err(ApiResponse::internal_server_error)?;

    // Send an invitation email to the new users
    let inviter = guard.get_user();
    for user in new_users {
        let recipient = PublicUser::from(&user);

        MailClient::no_reply()
            .send_invitation(&inviter, &recipient, form.space)
            .map_err(|e| ApiResponse::internal_server_error(format!("Coudn't send email: {e}")))?;
    }

    // Return success response
    Ok(ApiResponse::success(
        format!("{entries} users invited"),
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
