use std::fmt::{Display, Formatter};

use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*, sql_types::Text};
use rocket_sync_db_pools::diesel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Clone, Debug, Deserialize, Insertable, Queryable, QueryableByName, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    #[diesel(sql_type = Text)]
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub role: i16,
    pub status: i16,
    pub job_title: Option<String>,
    pub password: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn new(
        username: String,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
    ) -> Self {
        let timestamp = Utc::now().naive_utc();

        User {
            id: Uuid::new_v4(),
            username,
            first_name,
            last_name,
            email,
            phone: None,
            role: i16::from(UserRole::Reviewer),
            status: i16::from(UserStatus::Invited),
            job_title: None,
            password,
            bio: None,
            avatar_url: None,
            created_at: timestamp,
            updated_at: timestamp,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
#[diesel(table_name = users)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub role: i16,
    pub status: i16,
    pub job_title: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl PublicUser {
    pub fn from(user: &User) -> Self {
        PublicUser {
            id: user.id,
            username: user.username.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            phone: user.phone.clone(),
            role: user.role,
            status: user.status,
            job_title: user.job_title.clone(),
            bio: user.bio.clone(),
            avatar_url: user.avatar_url.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
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

#[derive(Debug, PartialEq)]
pub enum UserStatus {
    // User created but hasn't set a password yet)
    Invited = 0,
    /// User is inactive; currently not used
    Inactive = 1,
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
            1 => Ok(UserStatus::Inactive),
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
