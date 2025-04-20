use rocket::http::CookieJar;
use uuid::Uuid;

use crate::{
    api::{Error, Null},
    cookies,
    models::{
        users::{PublicUser, UserRole},
        workspaces::WorkspaceRole,
    },
};

use super::Policy;

impl Policy {
    pub fn create_workspaces(user: &PublicUser) -> Result<(), Error<Null>> {
        Policy::rule(user.is_at_least(UserRole::Manager)).authorize("User not allowed to create teams")
    }

    pub fn update_workspaces_info(
        workspace: Uuid,
        user: PublicUser,
        cookies: &CookieJar<'_>,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(user_is_at_least(WorkspaceRole::Contributor, workspace, cookies)?)
            .authorize("Not authorized to update workspace information")
    }

    pub fn update_workspaces_members(
        workspace: Uuid,
        user: PublicUser,
        cookies: &CookieJar<'_>,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(user_is_at_least(WorkspaceRole::Master, workspace, cookies)?)
            .authorize("Not authorized to add members")
    }

    pub fn remove_workspaces(
        workspace: Uuid,
        user: PublicUser,
        cookies: &CookieJar<'_>,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(user_is_at_least(WorkspaceRole::Master, workspace, cookies)?)
            .authorize("Not authorized to remove workspace")
    }
    
}

fn user_is_at_least(workspace_role: WorkspaceRole, workspace: Uuid, cookies: &CookieJar<'_>) -> Result<bool, Error<Null>> {
    let actual = cookies::workspaces::get_workspace_permission(cookies, workspace)?;
    Ok(actual >= i16::from(workspace_role))
}