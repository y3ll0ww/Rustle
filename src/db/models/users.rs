use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use rocket_sync_db_pools::diesel;
use rocket::serde::json::Json;
use uuid::Uuid;

use super::{DbConn, schemas::users};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)] // Table name is associated with the users table
pub struct User {
    pub user_id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[post("/users", format = "json", data = "<user>")]
pub async fn create_user(conn: DbConn, user: Json<User>) -> String {
    let mut new_user = user.into_inner(); // Extract user data from Json
    let username = new_user.username.clone();
    new_user.user_id = Uuid::new_v4().to_string(); // Generate a new UUID

    // Use Diesel to insert the new user
    let result = conn.run(move |c| {
        diesel::insert_into(users::table)
            .values(&new_user) // Clone new_user into the closure
            .execute(c) // Pass the connection
    }).await;

    match result {
        Ok(_) => format!("User {username} created"),
        Err(e) => format!("Error creating user: {e}"), // Print error details
    }
}