use std::fmt::{Display, Formatter};

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rocket_sync_db_pools::diesel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::user;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user)]
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

impl User {
    pub fn new(
        username: String,
        display_name: Option<String>,
        email: String,
        password_hash: String,
    ) -> Self {
        let timestamp = Utc::now().naive_utc();

        User {
            user_id: Uuid::new_v4().to_string(),
            user_role: UserRole::User.to_string(),
            username,
            display_name,
            email,
            password_hash,
            bio: None,
            avatar_url: None,
            created_at: timestamp,
            updated_at: timestamp,
        }
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
