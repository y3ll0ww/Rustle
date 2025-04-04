use std::fmt::{Display, Formatter};

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rocket_sync_db_pools::diesel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub role: i16,
    pub status: i16,
    pub username: String,
    pub display_name: Option<String>,
    pub email: String,
    pub password: String,
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
        password: String,
    ) -> Self {
        let timestamp = Utc::now().naive_utc();

        User {
            id: Uuid::new_v4(),
            role: i16::from(UserRole::Reviewer),
            status: i16::from(UserStatus::Invited),
            username,
            display_name,
            email,
            password,
            bio: None,
            avatar_url: None,
            created_at: timestamp,
            updated_at: timestamp,
        }
    }
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
#[diesel(table_name = users)]
pub struct PublicUser {
    pub id: Uuid,
    pub role: i16,
    pub status: i16,
    pub username: String,
    pub display_name: Option<String>,
    pub email: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl PublicUser {
    pub fn from(user: &User) -> Self {
        PublicUser {
            id: user.id,
            role: user.role,
            status: user.status,
            username: user.username.clone(),
            display_name: user.display_name.clone(),
            email: user.email.clone(),
            bio: user.bio.clone(),
            avatar_url: user.avatar_url.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }

    pub fn get_name(&self) -> String {
        self.display_name.clone().unwrap_or(self.username.clone())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum UserRole {
    Admin = 10,
    Manager = 5,
    Contributor = 1,
    Reviewer = 0,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl TryFrom<i16> for UserRole {
    type Error = String;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(UserRole::Admin),
            2 => Ok(UserRole::Manager),
            1 => Ok(UserRole::Contributor),
            0 => Ok(UserRole::Reviewer),
            _ => Err(format!("Invalid UserRole value: {value}")),
        }
    }
}

impl From<UserRole> for i16 {
    fn from(role: UserRole) -> Self {
        role as i16
    }
}

pub enum UserStatus {
    // User created but hasn't set a password yet)
    Invited = 0,
    /// User has set their password
    PasswordSet = 1,
    /// User is fully active, possibly after email verification
    Active = 2,
    /// User is suspended
    Suspended = 3,
    /// User is deleted
    Deleted = 4,
}

impl TryFrom<i16> for UserStatus {
    type Error = String;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UserStatus::Invited),
            1 => Ok(UserStatus::PasswordSet),
            2 => Ok(UserStatus::Active),
            3 => Ok(UserStatus::Suspended),
            4 => Ok(UserStatus::Deleted),
            _ => Err(format!("Invalid UserStatus value: {value}")),
        }
    }
}

impl From<UserStatus> for i16 {
    fn from(status: UserStatus) -> Self {
        status as i16
    }
}
