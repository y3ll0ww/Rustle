use rocket::{http::CookieJar, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::RedisMutex,
    cookies,
    database::{self, Db},
    models::workspaces::{Workspace, WorkspaceWithMembers},
    policies::Policy,
    routes::workspaces::get_workspace_with_members,
};

/// Returns an overview of workspaces of which the request user is a member.
#[get("/")]
pub async fn get_workspaces_of_current_user(
    guard: JwtGuard,
    db: Db,
) -> Result<Success<Vec<Workspace>>, Error<Null>> {
    let user = guard.get_user();

    // Retrieve all workspaces with the user ID
    let workspaces = database::workspaces::get_workspaces_by_user_id(&db, user.id).await?;

    // Return vector of workspaces
    Ok(ApiResponse::success(
        format!("Workspaces for '{}'", user.username),
        Some(workspaces),
    ))
}

/// Returns information about a workspace, including its members.
#[get("/<id>")]
pub async fn get_workspace_by_id(
    id: Uuid,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<WorkspaceWithMembers>, Error<Null>> {
    let user = guard.get_user();

    // Get the workspace information with its members
    let workspace_with_members = get_workspace_with_members(id, &db, redis).await?;

    // Return not found if the user is not an admin or a member
    Policy::workspaces_view(&user, &workspace_with_members)?;

    // Insert the workspace permissions in cookies
    if let Some(member) = workspace_with_members
        .members
        .iter()
        .find(|m| m.user.id == user.id)
    {
        cookies::permissions::insert_workspace_permission(id, member.role, cookies)?;
    }

    Ok(ApiResponse::success(
        format!(
            "Workspace '{}' from database",
            workspace_with_members.workspace.name
        ),
        Some(workspace_with_members),
    ))
}
