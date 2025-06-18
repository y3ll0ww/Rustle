use rocket::http::CookieJar;
use uuid::Uuid;

use crate::{
    api::{Error, Null},
    cookies,
    models::{
        users::{PublicUser, UserRole},
        workspaces::{WorkspaceRole, WorkspaceWithMembers},
    },
};

use super::Policy;

impl Policy {
    pub fn workspaces_view(
        user: &PublicUser,
        workspace_with_members: &WorkspaceWithMembers,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(user_is_member_of_workspace(user, workspace_with_members))
            .not_found("Workspace not found")
    }

    pub fn workspaces_create(user: &PublicUser) -> Result<(), Error<Null>> {
        Policy::rule(user.is_at_least(UserRole::Manager))
            .unauthorized("User not allowed to create teams")
    }

    pub fn workspaces_update_info(
        workspace: Uuid,
        user: PublicUser,
        cookies: &CookieJar<'_>,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(workspace_role_is_at_least(
                WorkspaceRole::Contributor,
                workspace,
                cookies,
            )?)
            .unauthorized("Not authorized to update workspace information")
    }

    pub fn workspaces_update_members(
        workspace: Uuid,
        user: PublicUser,
        cookies: &CookieJar<'_>,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(workspace_role_is_at_least(
                WorkspaceRole::Master,
                workspace,
                cookies,
            )?)
            .unauthorized("Not authorized to add members")
    }

    pub fn workspaces_remove(
        workspace: Uuid,
        user: PublicUser,
        cookies: &CookieJar<'_>,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(workspace_role_is_at_least(
                WorkspaceRole::Master,
                workspace,
                cookies,
            )?)
            .unauthorized("Not authorized to remove workspace")
    }
}

pub fn workspace_role_is_at_least(
    workspace_role: WorkspaceRole,
    workspace: Uuid,
    cookies: &CookieJar<'_>,
) -> Result<bool, Error<Null>> {
    let actual = cookies::permissions::get_workspace_permission(workspace, cookies).unwrap_or(-1);
    Ok(actual >= i16::from(workspace_role))
}

pub fn user_is_member_of_workspace(
    user: &PublicUser,
    workspace_with_members: &WorkspaceWithMembers,
) -> bool {
    workspace_with_members
        .members
        .iter()
        .any(|member| member.user.id == user.id)
}
