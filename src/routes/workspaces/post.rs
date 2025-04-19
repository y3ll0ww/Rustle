use crate::{cache, cookies, database, models::workspaces::NewWorkspace};

use super::*;

#[post("/new", data = "<form>")]
pub async fn create_new_workspace_by_form(
    form: Form<NewWorkspaceForm>,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
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

    // Add the workspace update timestamp to the cookies
    cookies::workspaces::add_workspace_timestamp(
        workspace_with_members.workspace.id,
        workspace_with_members.workspace.updated_at,
        cookies,
    );

    // Return success response
    Ok(ApiResponse::success(
        format!(
            "Workspace created: '{}'",
            workspace_with_members.workspace.name
        ),
        Some(workspace_with_members),
    ))
}

// Change owner
// Add member
