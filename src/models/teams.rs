use std::fmt::{Display, Formatter};

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rocket_sync_db_pools::diesel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{team_members, team_updates, teams};

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
    pub fn new(owner_id: String, team_name: String, team_description: Option<String>) -> Self {
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

#[derive(Insertable, Queryable, Serialize)]
#[diesel(table_name = team_members)]
pub struct TeamMember {
    pub team_id: String,
    pub user_id: String,
    pub team_privilege: i32,
}

#[derive(Clone, Deserialize, Insertable, Queryable, Serialize)]
#[diesel(table_name = team_updates)]
pub struct TeamUpdate {
    pub team_id: String,
    pub last_updated: String,
}

#[derive(Deserialize, Queryable, Serialize)]
pub struct TeamMemberInfo {
    pub user_id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub team_privilege: i32,
}

#[derive(Deserialize, Serialize)]
pub struct TeamWithMembers {
    pub team: Team,
    pub members: Vec<TeamMemberInfo>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum TeamRole {
    /// Maximum privileges; only one able to delete a team
    Owner = 10,
    /// High-level privileges; can manage members, settings, and permissions but cannot delete the team
    Master = 5,
    /// Can contribute work but has limited administrative privileges
    Contributor = 2,
    /// Can review and approve work but cannot make direct contributions
    Stakeholder = 1,
    /// Limited access, can only view but not interact with content
    Viewer = 0,
}

impl Display for TeamRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl TryFrom<i32> for TeamRole {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            10 => Ok(TeamRole::Owner),
            5 => Ok(TeamRole::Master),
            2 => Ok(TeamRole::Contributor),
            1 => Ok(TeamRole::Stakeholder),
            0 => Ok(TeamRole::Viewer),
            _ => Err(format!("Invalid TeamRole value: {value}")),
        }
    }
}
