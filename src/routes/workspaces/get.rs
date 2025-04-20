use rocket::{http::CookieJar, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    cookies,
    database::{self, Db},
    models::workspaces::{Workspace, WorkspaceWithMembers},
};

/// Returns an overview of workspaces of which the request user is a member.
#[get("/")]
pub async fn get_list_of_workspaces_by_user_id(
    guard: JwtGuard,
    db: Db,
) -> Result<Success<Vec<Workspace>>, Error<Null>> {
    let user_id = guard.get_user().id;

    // Retrieve all workspaces with the user ID
    let workspaces = database::workspaces::get_workspaces_by_user_id(&db, user_id).await?;

    // Return vector of workspaces
    Ok(ApiResponse::success(
        format!("Workspaces for user '{user_id}'"),
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
    // Check the cache for an existing workspace with members
    let workspace_with_members = match cache::workspaces::get_workspace_cache(redis, id).await? {
        Some(cached_workspace) => cached_workspace,
        None => {
            // Get the workspace with members from the database
            let workspace_from_database =
                database::workspaces::get_workspace_by_id(&db, id).await?;

            // Add the workspace with members to the cache
            cache::workspaces::add_workspace_cache(redis, &workspace_from_database).await;

            // Return a fresh workspace with members from the database
            workspace_from_database
        }
    };

    // Insert the permission in cookies
    let user_id = guard.get_user().id;
    if let Some(member) = workspace_with_members
        .members
        .iter()
        .find(|m| m.user.id == user_id)
    {
        cookies::workspaces::insert_workspace_permission(cookies, id, member.role)?;
    }

    Ok(ApiResponse::success(
        format!(
            "Workspace '{}' from database",
            workspace_with_members.workspace.name
        ),
        Some(workspace_with_members),
    ))
}
