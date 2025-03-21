use std::fmt::{Display, Formatter};

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rocket_sync_db_pools::diesel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{forms::teams::UpdateTeamForm, schema::{team_members, team_updates, teams}};

#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[diesel(table_name = teams)]
pub struct Team {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub team_name: String,
    pub team_description: Option<String>,
    pub image_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Team {
    pub fn new(owner_id: Uuid, team_name: String, team_description: Option<String>) -> Self {
        let timestamp = Utc::now().naive_utc();

        Team {
            id: Uuid::new_v4(),
            owner_id,
            team_name,
            team_description,
            image_url: None,
            created_at: timestamp,
            updated_at: timestamp,
        }
    }

    pub fn update(&mut self, update_team_form: UpdateTeamForm) {
        if let Some(team_name) = update_team_form.team_name {
            self.team_name = team_name
        }

        if let Some(description) = update_team_form.team_description {
            self.team_description = Some(description)
        }

        if let Some(image_url) = update_team_form.image_url {
            self.image_url = Some(image_url)
        }
    }
}

#[derive(Insertable, Queryable, Serialize)]
#[diesel(table_name = team_members)]
pub struct TeamMember {
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub team_role: i16,
}

#[derive(Clone, Deserialize, Insertable, Queryable, Serialize)]
#[diesel(table_name = team_updates)]
pub struct TeamUpdate {
    pub team_id: Uuid,
    pub last_updated: NaiveDateTime,
}

#[derive(Deserialize, Queryable, Serialize)]
pub struct TeamMemberInfo {
    pub user_id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub team_role: i16,
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

impl TryFrom<i16> for TeamRole {
    type Error = String;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
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
