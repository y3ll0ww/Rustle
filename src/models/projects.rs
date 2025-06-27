use std::fmt::{Display, Formatter};

use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket_sync_db_pools::diesel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    forms::projects::NewProjectForm,
    models::MemberInfo,
    schema::{project_members, projects},
};

#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[diesel(table_name = projects)]
pub struct Project {
    pub id: Uuid,
    pub workspace: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub member_count: i32,
    pub image_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable, Queryable, Serialize)]
#[diesel(table_name = project_members)]
pub struct ProjectMember {
    pub project: Uuid,
    pub member: Uuid,
    pub role: i16,
}

#[derive(Deserialize, Serialize)]
pub struct ProjectWithMembers {
    pub project: Project,
    pub members: Vec<MemberInfo>,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject {
    pub name: String,
    pub description: Option<String>,
}

impl NewProject {
    pub fn from_form(form: NewProjectForm) -> Self {
        NewProject {
            name: form.name,
            description: form.description,
        }
    }
}

#[derive(AsChangeset, Clone, Deserialize, Serialize)]
#[diesel(table_name = projects)]
pub struct ProjectUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ProjectRole {
    /// Maximum privileges; only one able to delete a workspace
    Owner = 10,
    /// High-level privileges; can manage members, settings, and permissions but cannot delete the workspace
    Master = 5,
    /// Can contribute work but has limited administrative privileges
    Contributor = 2,
    /// Can review and approve work but cannot make direct contributions
    Stakeholder = 1,
    /// Limited access, can only view but not interact with content
    Viewer = 0,
}

impl Display for ProjectRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl TryFrom<i16> for ProjectRole {
    type Error = String;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            10 => Ok(ProjectRole::Owner),
            5 => Ok(ProjectRole::Master),
            2 => Ok(ProjectRole::Contributor),
            1 => Ok(ProjectRole::Stakeholder),
            0 => Ok(ProjectRole::Viewer),
            _ => Err(format!("Invalid ProjectRole value: {value}")),
        }
    }
}

impl From<ProjectRole> for i16 {
    fn from(role: ProjectRole) -> Self {
        role as i16
    }
}
