use std::fmt::{Display, Formatter};

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rocket_sync_db_pools::diesel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub privilege: i32,
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
            id: Uuid::new_v4().to_string(),
            privilege: UserRole::Reviewer as i32,
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

#[derive(Debug, Deserialize, Serialize)]
pub enum UserRole {
    Admin = 3,
    Manager = 2,
    Contributor = 1,
    Reviewer = 0,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl TryFrom<i32> for UserRole {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(UserRole::Admin),
            2 => Ok(UserRole::Manager),
            1 => Ok(UserRole::Contributor),
            0 => Ok(UserRole::Reviewer),
            _ => Err(format!("Invalid UserRole value: {value}")),
        }
    }
}
