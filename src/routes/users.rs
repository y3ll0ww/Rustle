use diesel::{
    prelude::*,
    result::{DatabaseErrorKind, Error as DieselError},
};

use redis::AsyncCommands;
use rocket::{form::Form, serde::json::Json, State};
use rocket_sync_db_pools::diesel;
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::{create_token, AuthenticatedUser},
    db::Database,
    forms::users::{LoginForm, NewUserForm, Password},
    models::users::User,
    redis::RedisMutex,
    schema::users,
};

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
#[post("/form", data = "<form>")]
pub async fn submit<'r>(
    db: Database,
    form: Form<NewUserForm<'r>>,
    redis_pool: &State<RedisMutex>,
) -> Result<Success<String>, Error<Null>> {
    // Hash the provided password
    let password_hash = form
        .password
        .hash_password()
        .map_err(|e| ApiResponse::internal_server_error(format!("Couldn't hash password: {e}")))?;

    // Create a new User
    let new_user = User::new(
        form.username.to_string(),
        Some(form.username.to_string()),
        form.email.to_string(),
        password_hash,
    );

    // Clone information for later use
    let user_id = new_user.id.clone();
    let username = new_user.username.clone();
    let user_privilege = new_user.privilege;

    // Add the new User to the database
    db.run(move |conn| {
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)
    })
    .await
    .map_err(|e| match e {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            return ApiResponse::conflict("User already exists".to_string(), e.to_string());
        }
        _ => ApiResponse::internal_server_error(format!("Error creating user: {}", e)),
    })?;

    // Pass user info to create_token for caching
    let token = create_token(user_id, username.clone(), user_privilege, redis_pool)
        .await
        .map_err(|e| ApiResponse::internal_server_error(e))?;

    // Return success response
    Ok(ApiResponse::success(
        format!("User '{username}' created succesfully"),
        Some(token),
    ))
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
#[delete("/delete/<id>")]
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
    redis_pool: &State<RedisMutex>,
) -> Result<Success<String>, Error<Null>> {
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

    // Pass user info to create token for caching
    let token = create_token(
        user.id.clone(),
        user.username.clone(),
        user.privilege,
        redis_pool,
    )
    .await
    .map_err(|e| ApiResponse::internal_server_error(e))?;

    // Return the token
    Ok(ApiResponse::success(
        "Login successful".to_string(),
        Some(token),
    ))
}

#[post("/logout")]
pub async fn logout(user: AuthenticatedUser, redis_pool: &State<RedisMutex>) -> Success<String> {
    let mut redis = redis_pool.lock().await.get_connection().await.unwrap();

    // Remove token from Redis
    let _: () = redis.del(&user.token).await.unwrap();

    // Remove user info from Redis
    let user_key = format!("user:{}", user.id);
    let _: () = redis.del(user_key).await.unwrap();

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
