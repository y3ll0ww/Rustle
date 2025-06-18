use rocket::http::CookieJar;
use uuid::Uuid;

use crate::{
    api::{Error, Null},
    models::{
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
}
