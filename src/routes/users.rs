use diesel::{
    prelude::*,
    result::{DatabaseErrorKind, Error as DieselError},
};

use rocket::{form::Form, http::CookieJar, serde::json::Json};
use rocket_sync_db_pools::diesel;
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cookies::{users::generate_and_add_cookies, TOKEN_COOKIE, USER_COOKIE},
    db::Database,
    forms::users::{LoginForm, NewUserForm, Password},
    models::users::User,
    schema::users,
};

#[get("/")]
pub async fn all(db: Database) -> Result<Success<Vec<User>>, Error<Null>> {
    db.run(move |conn| {
        users::table.get_results::<User>(conn)
        //.filter(users::username.eq(username))
        //.first::<User>(conn)
    })
    .await
    .map(|user| ApiResponse::success("Returning all users".to_string(), Some(user)))
    .map_err(|e| ApiResponse::not_found(e.to_string()))
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
pub async fn register(
    db: Database,
    form: Form<NewUserForm<'_>>,
    cookies: &CookieJar<'_>,
) -> Result<Success<Null>, Error<Null>> {
    // Hash the provided password
    let password_hash = form
        .password
        .hash_password()
        .map_err(|e| ApiResponse::internal_server_error(format!("Couldn't hash password: {e}")))?;

    // Create a new User
    let new_user = User::new(
        form.username.to_string(),
        None,
        form.email.to_string(),
        password_hash,
    );

    // Clone information for later use
    let user_id = new_user.id.clone();
    let username = new_user.username.clone();
    let privilege = new_user.privilege;

    // Add the new User to the database
    db.run(move |conn| {
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)
    })
    .await
    .map_err(|e| match e {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            ApiResponse::conflict("User already exists".to_string(), e.to_string())
        }
        _ => ApiResponse::internal_server_error(format!("Error creating user: {}", e)),
    })?;

    generate_and_add_cookies(user_id, username.clone(), privilege, cookies).await?;

    // Return success response
    Ok(ApiResponse::success(
        format!("User '{username}' created succesfully"),
        None,
    ))
}

/// Deletes a [`User`] from the database.
///
/// **Route**: `./delete/<id>`
///
/// ### Parameters
/// * `db`: Instance of the [`Database`] connection.
/// * `id`: The ID from the [`User`] to be deleted.
///
/// ### Returns
/// * `Ok(Success<String>)`: When `Ok`, it returns a wrapped in [`Success`] with [`Null`] data.
/// * `Err(Error<String>)`: When `Err`, it returns an [`Error`] with [`Null`] data.
#[delete("/<id>/delete")]
pub async fn delete(db: Database, id: String) -> Result<Success<Null>, Error<Null>> {
    // Define the response messages beforehand
    let success_msg = format!("User with ID '{id}' successfully deleted");
    let failed_msg = format!("User with ID '{id}' not removed");

    // Delete the records from the database and collect the number of deleted rows
    let deleted_rows = db
        .run(move |conn| diesel::delete(users::table.filter(users::id.eq(id))).execute(conn))
        .await
        .map_err(|e| match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                ApiResponse::bad_request(format!("{}: {:?} - {:?}", failed_msg, kind, info))
            }
            other => ApiResponse::internal_server_error(format!("{}: {}", failed_msg, other)),
        })?;

    // If there are any deleted rows, it means the user is successfully deleted
    if deleted_rows > 0 {
        Ok(ApiResponse::success(success_msg, None))
    } else {
        Err(ApiResponse::not_found(format!(
            "{}: No user found with that ID",
            failed_msg
        )))
    }
}

#[get("/<username>")]
pub async fn get(db: Database, username: String) -> Result<Success<User>, Error<Null>> {
    // Only get the user from the database
    get_user(db, username).await
}

#[post("/login", data = "<credentials>")]
pub async fn login(
    db: Database,
    credentials: Form<LoginForm<'_>>,
    cookies: &CookieJar<'_>,
) -> Result<Success<Null>, Error<Null>> {
    // Get the user from the database
    let user = match &get_user(db, credentials.username.to_string()).await?.data {
        Some(user) => user.clone(),
        None => {
            return Err(ApiResponse::internal_server_error(
                "No user data".to_string(),
            ));
        }
    };

    // Validate if the given password is correct
    if !Password::verify_password(credentials.password, &user.password_hash).map_err(|e| {
        ApiResponse::internal_server_error(format!("Password verification failed: {}", e))
    })? {
        return Err(ApiResponse::bad_request("Invalid password".to_string()));
    };

    generate_and_add_cookies(user.id, user.username, user.privilege, cookies).await?;

    // Return the token
    Ok(ApiResponse::success("Login successful".to_string(), None))
}

#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>, _guard: JwtGuard) -> Success<String> {
    cookies.remove_private(TOKEN_COOKIE);
    cookies.remove_private(USER_COOKIE);

    ApiResponse::success(
        "Logout successful - token and user info removed".to_string(),
        None,
    )
}

async fn get_user(db: Database, username: String) -> Result<Success<User>, Error<Null>> {
    db.run(move |conn| {
        users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
    })
    .await
    .map(|user| ApiResponse::success(format!("User '{}' found", user.username), Some(user)))
    .map_err(|e| ApiResponse::not_found(e.to_string()))
}

#[post("/create", format = "json", data = "<user>")]
pub async fn create(db: Database, user: Json<User>) -> String {
    let mut new_user = user.into_inner(); // Extract user data from Json
    let username = new_user.username.clone();
    new_user.id = Uuid::new_v4().to_string(); // Generate a new UUID

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
