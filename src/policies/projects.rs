use rocket::http::CookieJar;
use uuid::Uuid;

use crate::{
    api::{Error, Null},
    models::{users::PublicUser, workspaces::WorkspaceRole},
    policies::workspaces::workspace_role_is_at_least,
};

use super::Policy;

impl Policy {
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
            .authorize("Not authorized to create new projects in this workspace")
    }
}
