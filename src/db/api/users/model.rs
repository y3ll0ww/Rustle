use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket_sync_db_pools::diesel;
use serde::{Deserialize, Serialize};

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

table! {
    users (user_id) {
        user_id -> VarChar,
        user_role -> VarChar,
        username -> Varchar,
        display_name -> Nullable<Varchar>,
        email -> Varchar,
        password_hash -> Varchar,
        bio -> Nullable<Text>,
        avatar_url -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
