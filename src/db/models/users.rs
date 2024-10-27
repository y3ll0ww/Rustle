use std::fmt::{Display, Formatter};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use rocket::{
    http::{ContentType, Status},
    local::blocking::Client,
    serde::json::Json,
};
use rocket_sync_db_pools::diesel;
use serde_json::json;
use uuid::Uuid;

use super::{schemas::users, Db};

#[cfg(test)]
mod tests;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)] // Table name is associated with the users table
pub struct User {
    pub user_id: String,
    pub user_role: String,
    pub username: String,
    pub display_name: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UserRole {
    Admin,
    User,
    Guest,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::User => write!(f, "user"),
            UserRole::Guest => write!(f, "guest"),
        }
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
    let deleted_count = db.run(move |conn| {
        diesel::delete(users::table.filter(users::user_id.eq(id)))
            .execute(conn)
    }).await.ok();

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
