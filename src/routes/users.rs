use diesel::prelude::*;

use rocket::{form::Form, serde::json::Json};
use rocket_sync_db_pools::diesel;
use uuid::Uuid;

use crate::{
    api::{ApiResponse, JsonResponse},
    db::Database,
    forms::users::{InsertedUser, NewUser},
    models::users::User,
    schema::users,
};

/// This function allows for the creation of a new [`User`] by using a form.
///
/// **Route**: `./form`
///
/// ### Parameters
/// * `db`: Instance of the [`Database`] connection.
/// * `form`: A [`Form`] with [`NewUser`] information to create a [`User`].
///
/// ### Returns
/// * `Ok(JsonResponse<InsertedUser>)`: When `Ok`, it returns a [`JsonResponse`] with the [`InsertedUser`].
/// * `Err(JsonResponse<User>)`: When `Err`, it returns a [`JsonResponse`] with `None` data.
#[post("/form", data = "<form>")]
pub async fn submit<'r>(
    db: Database,
    form: Form<NewUser<'r>>,
) -> Result<JsonResponse<InsertedUser>, JsonResponse<String>> {
    // Hash the provided password
    let password = form.password.hash_password().map_err(|e| {
        ApiResponse::internal_server_error(format!("Couldn't hash password: {}", e))
    })?;

    // Create a new User
    let new_user = User::new(
        form.username.to_string(),
        form.display_name.map(|s| s.to_string()),
        form.email.to_string(),
        password,
    );

    // Create a new InsertedUser in case the function executes succesfully
    let inserted_user = InsertedUser::from_user(&new_user);

    // Add the new User to the database
    db.run(move |conn| {
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)
    })
    .await
    .map_err(|e| ApiResponse::internal_server_error(format!("Error creating user: {}", e)))?;

    // Return success response
    Ok(ApiResponse::success(
        format!(
            "User '{}' created succesfully",
            inserted_user.username.clone()
        ),
        Some(inserted_user),
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
/// * `Ok(JsonResponse<String>)`: When `Ok`, it returns a wrapped in an [`JsonResponse`] with `None` data.
/// * `Err(JsonRepsonse<String>)`: When `Err`, it returns an [`JsonResponse`] with `None` data.
#[delete("/delete/<id>")]
pub async fn delete(
    db: Database,
    id: String,
) -> Result<JsonResponse<String>, JsonResponse<String>> {
    let success_msg = format!("User with ID '{}' successfully deleted", id);
    let failed_msg = format!("User with ID '{}' not removed", id);

    let deleted_rows = db
        .run(move |conn| diesel::delete(users::table.filter(users::id.eq(id))).execute(conn))
        .await
        .map_err(|e| match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                ApiResponse::bad_request(format!("{}: {:?} - {:?}", failed_msg, kind, info))
            }
            other => ApiResponse::internal_server_error(format!("{}: {}", failed_msg, other)),
        })?;

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
pub async fn get(
    db: Database,
    username: String,
) -> Result<JsonResponse<User>, JsonResponse<String>> {
    let success = format!("User '{username}' found");

    db.run(move |conn| {
        users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
    })
    .await
    .map(|user| ApiResponse::success(success, Some(user)))
    .map_err(|e| ApiResponse::not_found(e.to_string()))
}
