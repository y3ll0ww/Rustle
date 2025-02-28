use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rocket_sync_db_pools::diesel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::teams;

#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[diesel(table_name = teams)]
pub struct Team {
    pub id: String,
    pub owner_id: String,
    pub team_name: String,
    pub team_description: Option<String>,
    pub image_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Team {
    pub fn new(
        owner_id: String,
        team_name: String,
        team_description: Option<String>,
    ) -> Self {
        let timestamp = Utc::now().naive_utc();

        Team {
            id: Uuid::new_v4().to_string(),
            owner_id,
            team_name,
            team_description,
            image_url: None,
            created_at: timestamp,
            updated_at: timestamp,
        }
    }
}
