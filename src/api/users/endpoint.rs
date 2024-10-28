use chrono::Utc;
use diesel::prelude::*;

use rocket::{form::Form, http::Status, serde::json::Json};
use rocket_sync_db_pools::diesel;
use uuid::Uuid;

use super::{form::Account, form::UserRole, model::users, model::User};
use crate::db::Db;

#[post("/form", data = "<form>")]
pub async fn submit<'r>(db: Db, form: Form<Account<'r>>) -> (Status, String) {
    let password = match form.password.hash_password() {
        Ok(hash) => hash,
        Err(e) => {
            return (
                Status { code: 500 },
                format!("Couldn't hash the password: {e}"),
            )
        }
    };

    let timestamp = Utc::now().naive_utc();

    let new_user = User {
        user_id: Uuid::new_v4().to_string(),
        user_role: UserRole::User.to_string(),
        username: form.username.to_string(),
        display_name: form.display_name.map(|s| s.to_string()),
        email: form.email.to_string(),
        password_hash: password,
        bio: form.bio.map(|s| s.to_string()),
        avatar_url: form.avatar_url.map(|s| s.to_string()),
        created_at: timestamp,
        updated_at: timestamp,
    };

    // Use Diesel to insert the new user
    let result = db
        .run(move |c| {
            diesel::insert_into(users::table)
                .values(&new_user) // Clone new_user into the closure
                .execute(c) // Pass the connection
        })
        .await;

    match result {
        Ok(_) => (
            Status { code: 200 },
            format!("User {} created", form.username),
        ),
        Err(e) => (Status { code: 500 }, format!("Error creating user: {e}")),
    }
}

#[post("/create", format = "json", data = "<user>")]
pub async fn create_user(db: Db, user: Json<User>) -> String {
    let mut new_user = user.into_inner(); // Extract user data from Json
    let username = new_user.username.clone();
    new_user.user_id = Uuid::new_v4().to_string(); // Generate a new UUID

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

#[delete("/delete/<id>")]
pub async fn delete_user(db: Db, id: String) -> String {
    let deleted_count = db
        .run(move |conn| diesel::delete(users::table.filter(users::user_id.eq(id))).execute(conn))
        .await
        .ok();

    println!("{deleted_count:?}");

    if deleted_count.is_some() && deleted_count.unwrap() > 0 {
        "User deleted successfully".to_string()
    } else {
        "User not found".to_string()
    }
}

#[get("/<username>")]
pub async fn get_user(db: Db, username: String) -> String {
    let name = username.clone();
    let user: Option<Json<User>> = db
        .run(move |conn| {
            users::table
                .filter(users::username.eq(username))
                .first(conn)
        })
        .await
        .map(Json)
        .ok();

    match user {
        Some(json) => format!("{:?}", json.0),
        None => format!("'{name}' not found..."),
    }
}
