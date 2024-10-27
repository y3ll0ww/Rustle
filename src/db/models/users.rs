use std::fmt::{Display, Formatter};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use rocket::{
    form::Form,
    http::Status,
    serde::json::Json,
};
use rocket_sync_db_pools::diesel;
use uuid::Uuid;

use super::{schemas::users, Db};

#[cfg(test)]
mod tests;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
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

#[derive(Debug, FromForm, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Account<'v> {
    #[field(validate = len(1..))]
    username: &'v str,
    display_name: Option<&'v str>,
    password: Password<'v>,
    #[field(validate = contains('@').or_else(msg!("invalid email address")))]
    email: &'v str,
    bio: Option<&'v str>,
    avatar_url: Option<&'v str>,
}

#[derive(Debug, FromForm, Serialize, Deserialize)]
struct Password<'v> {
    #[field(validate = len(6..))]
    #[field(validate = eq(self.second))]
    #[allow(unused)]
    first: &'v str,
    #[allow(unused)]
    #[field(validate = eq(self.first))]
    second: &'v str,
}

impl<'v> Password<'v> {
    pub fn hash_password(&self) -> Result<String, argon2::password_hash::Error> {
        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        // Hash password to PHC string ($argon2id$v=19$...)
        Ok(argon2
            .hash_password(self.first.as_bytes(), &salt)?
            .to_string())
    }
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
        Ok(_) => (Status { code: 200 }, format!("User {} created", form.username)),
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
