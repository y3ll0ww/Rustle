use std::fmt::{Display, Formatter};

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rocket_sync_db_pools::diesel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    forms::workspace::NewWorkspaceForm,
    schema::{workspace_members, workspaces},
};

use super::MemberInfo;

#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[diesel(table_name = workspaces)]
pub struct Workspace {
    pub id: Uuid,
    pub owner: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub member_count: i32,
    pub image_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Workspace {
    pub fn new(owner: Uuid, name: String, description: Option<String>) -> Self {
        let timestamp = Utc::now().naive_utc();

        Workspace {
            id: Uuid::new_v4(),
            owner,
            name,
            description,
            member_count: 0,
            image_url: None,
            created_at: timestamp,
            updated_at: timestamp,
        }
    }

    pub fn update(&mut self, workspace_update: WorkspaceUpdate) {
        if let Some(name) = workspace_update.name {
            self.name = name
        }

        if let Some(description) = workspace_update.description {
            self.description = Some(description)
        }

        if let Some(image_url) = workspace_update.image_url {
            self.image_url = Some(image_url)
        }
    }
}

#[derive(Deserialize, Insertable, Serialize)]
#[diesel(table_name = workspaces)]
pub struct WorkspaceNew {
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(AsChangeset, Clone, Deserialize, Serialize)]
#[diesel(table_name = workspaces)]
pub struct WorkspaceUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Deserialize, Insertable, Queryable, Serialize)]
#[diesel(table_name = workspace_members)]
pub struct WorkspaceMember {
    pub workspace: Uuid,
    pub member: Uuid,
    pub role: i16,
}

#[derive(Deserialize, Serialize)]
pub struct WorkspaceWithMembers {
    pub workspace: Workspace,
    pub members: Vec<MemberInfo>,
}

#[derive(Insertable)]
#[diesel(table_name = workspaces)]
pub struct NewWorkspace {
    pub owner: Uuid,
    name: String,
    description: Option<String>,
}

impl NewWorkspace {
    pub fn from_form(owner: Uuid, form: NewWorkspaceForm) -> Self {
        NewWorkspace {
            owner,
            name: form.name,
            description: form.description,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum WorkspaceRole {
    /// Maximum privileges; only one able to delete a workspace
    Owner = 10,
    /// High-level privileges; can manage members, settings, and permissions but cannot delete the workspace
    Manager = 5,
    /// Can contribute work but has limited administrative privileges
    Contributor = 2,
    /// Can review and approve work but cannot make direct contributions
    Stakeholder = 1,
    /// Limited access, can only view but not interact with content
    Viewer = 0,
}

impl Display for WorkspaceRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl TryFrom<i16> for WorkspaceRole {
    type Error = String;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            10 => Ok(WorkspaceRole::Owner),
            5 => Ok(WorkspaceRole::Manager),
            2 => Ok(WorkspaceRole::Contributor),
            1 => Ok(WorkspaceRole::Stakeholder),
            0 => Ok(WorkspaceRole::Viewer),
            _ => Err(format!("Invalid WorkspaceRole value: {value}")),
        }
    }
}

impl From<WorkspaceRole> for i16 {
    fn from(role: WorkspaceRole) -> Self {
        role as i16
    }
}
