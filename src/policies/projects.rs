use rocket::http::CookieJar;
use uuid::Uuid;

use crate::{
    api::{Error, Null},
    cookies,
    models::{
        projects::ProjectRole,
        users::PublicUser,
        workspaces::{WorkspaceRole, WorkspaceWithMembers},
    },
    policies::workspaces::{user_is_member_of_workspace, workspace_role_is_at_least},
};

use super::Policy;

/// PROJECT PERMISSIONS:
///
/// 1. Projects: C -> WorkspaceRole Master
/// 2. Projects: R -> ProjectRole Viewer / Admin
/// 3. Projects: U -> ProjectRole Contributer / Admin
/// 4. Project members: U -> ProjectRole Manager / Admin
/// 5. Project members: D -> WorkspaceRole Manager / Admin
impl Policy {
    /// [`Admin`](crate::models::users::UserRole::Admin) or
    /// [`Viewer`](ProjectRole::Viewer)+
    pub fn projects_view(
        user: &PublicUser,
        workspace_with_members: &WorkspaceWithMembers,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(user_is_member_of_workspace(user, workspace_with_members))
            .not_found("Project not found")
    }

    /// [`Admin`](crate::models::users::UserRole::Admin) or
    /// [`Manager`](WorkspaceRole::Manager)+
    pub fn projects_create(
        workspace: Uuid,
        user: PublicUser,
        cookies: &CookieJar<'_>,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(workspace_role_is_at_least(
                WorkspaceRole::Manager,
                workspace,
                cookies,
            )?)
            .unauthorized("Not authorized to create new projects in this workspace")
    }

    /// [`Admin`](crate::models::users::UserRole::Admin) or
    /// [`Contributor`](ProjectRole::Contributor)+
    pub fn projects_update_info(
        project: Uuid,
        user: PublicUser,
        cookies: &CookieJar<'_>,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(project_role_is_at_least(
                ProjectRole::Contributor,
                project,
                cookies,
            )?)
            .unauthorized("Not authorized to update project information")
    }

    /// [`Admin`](crate::models::users::UserRole::Admin) or
    /// [`Manager`](WorkspaceRole::Manager)+
    pub fn projects_remove(
        workspace: Uuid,
        user: PublicUser,
        cookies: &CookieJar<'_>,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(workspace_role_is_at_least(
                WorkspaceRole::Owner,
                workspace,
                cookies,
            )?)
            .unauthorized("Not authorized to remove project")
    }

    /// [`Admin`](crate::models::users::UserRole::Admin) or
    /// [`Master`](ProjectRole::Master)+
    pub fn project_update_members(
        project: Uuid,
        user: PublicUser,
        cookies: &CookieJar<'_>,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(project_role_is_at_least(
                ProjectRole::Master,
                project,
                cookies,
            )?)
            .unauthorized("Not authorized to add members")
    }
}

pub fn project_role_is_at_least(
    project_role: ProjectRole,
    project: Uuid,
    cookies: &CookieJar<'_>,
) -> Result<bool, Error<Null>> {
    let actual = cookies::permissions::get_project_permission(project, cookies).unwrap_or(-1);
    Ok(actual >= i16::from(project_role))
}
