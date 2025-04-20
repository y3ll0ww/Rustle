use rocket::{http::CookieJar, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    database::{self, Db},
    models::workspaces::WorkspaceWithMembers,
    policies::Policy,
};

/// Deletes a [`Workspace`] and related
/// [`WorkspaceMember`](crate::models::workspaces::WorkspaceMember)s.
#[delete("/<id>/delete")]
pub async fn delete_workspace_by_id(
    id: Uuid,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    Policy::remove_workspaces(id, guard.get_user(), cookies)?;

    // Remove the workspace from the database (relevant records
    // in different tables will be cascaded by Postgres)
    let workspace = database::workspaces::remove_workspace(&db, id).await?;

    // Remove the workspace from the cache
    cache::workspaces::remove_workspace_cache(redis, workspace.id).await;

    // Return success
    Ok(ApiResponse::success(
        format!("Workspace {} deleted", workspace.id),
        None,
    ))
}

#[delete("/<id>/remove-member/<member>")]
pub async fn remove_member_from_workspace(
    id: Uuid,
    member: Uuid,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<WorkspaceWithMembers>, Error<Null>> {
    Policy::update_workspaces_members(id, guard.get_user(), cookies)?;

    // Remove the member from the workspace
    let workspace_with_members =
        database::workspaces::remove_member_from_workspace(&db, id, member).await?;

    // Update the workspace in the cache
    cache::workspaces::add_workspace_cache(redis, &workspace_with_members).await;

    // Return success
    Ok(ApiResponse::success(
        format!("Member '{member}' removed"),
        Some(workspace_with_members),
    ))
}
