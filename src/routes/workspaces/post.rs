use rocket::{form::Form, State};

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    database::{self, Db},
    forms::workspace::NewWorkspaceForm,
    models::{
        users::UserRole,
        workspaces::{NewWorkspace, WorkspaceWithMembers},
    },
};

#[post("/new", data = "<form>")]
pub async fn create_new_workspace_by_form(
    form: Form<NewWorkspaceForm>,
    guard: JwtGuard,
    redis: &State<RedisMutex>,
    db: Db,
) -> Result<Success<WorkspaceWithMembers>, Error<Null>> {
    // Get user information from cookies
    let user = guard.get_user();

    // Validate user permissions
    if user.role < i16::from(UserRole::Manager) {
        return Err(ApiResponse::unauthorized(
            "User not allowed to create teams".to_string(),
        ));
    }

    // Insert and return a new workspace with members
    let workspace_with_members = database::workspaces::insert_new_workspace(
        &db,
        NewWorkspace::from_form(user.id, form.into_inner()),
    )
    .await?;

    // Add the workspace information to the cache
    cache::workspaces::add_workspace_cache(redis, &workspace_with_members).await;

    // Return success response
    Ok(ApiResponse::success(
        format!(
            "Workspace created: '{}'",
            workspace_with_members.workspace.name
        ),
        Some(workspace_with_members),
    ))
}
