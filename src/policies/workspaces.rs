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
    pub fn workspaces_create(user: &PublicUser) -> Result<(), Error<Null>> {
        Policy::rule(user.is_at_least(UserRole::Manager))
            .authorize("User not allowed to create teams")
    }

    pub fn workspaces_update_info(
        workspace: Uuid,
        user: PublicUser,
        cookies: &CookieJar<'_>,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(user_is_at_least(
                WorkspaceRole::Contributor,
                workspace,
                cookies,
            )?)
            .authorize("Not authorized to update workspace information")
    }

    pub fn workspaces_update_members(
        workspace: Uuid,
        user: PublicUser,
        cookies: &CookieJar<'_>,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(user_is_at_least(WorkspaceRole::Master, workspace, cookies)?)
            .authorize("Not authorized to add members")
    }

    pub fn workspaces_remove(
        workspace: Uuid,
        user: PublicUser,
        cookies: &CookieJar<'_>,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(user_is_at_least(WorkspaceRole::Master, workspace, cookies)?)
            .authorize("Not authorized to remove workspace")
    }
}

fn user_is_at_least(
    workspace_role: WorkspaceRole,
    workspace: Uuid,
    cookies: &CookieJar<'_>,
) -> Result<bool, Error<Null>> {
    let actual = cookies::workspaces::get_workspace_permission(cookies, workspace).unwrap_or(-1);
    Ok(actual >= i16::from(workspace_role))
}
