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

impl Policy {
    pub fn projects_view(
        user: &PublicUser,
        workspace_with_members: &WorkspaceWithMembers,
    ) -> Result<(), Error<Null>> {
        Policy::rule(user.is_admin())
            .or(user_is_member_of_workspace(user, workspace_with_members))
            .not_found("Project not found")
    }

    pub fn projects_create(
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
            .unauthorized("Not authorized to create new projects in this workspace")
    }
}
