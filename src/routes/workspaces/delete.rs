use rocket::State;
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    database::{self, Db},
};

/// Deletes a [`Workspace`] and related
/// [`WorkspaceMember`](crate::models::workspaces::WorkspaceMember)s.
#[delete("/<id>/delete")]
pub async fn delete_workspace_by_id(
    id: Uuid,
    guard: JwtGuard,
    db: Db,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    // Remove the workspace from the database (relevant records
    // in different tables will be cascaded by Postgres)
    let workspace = database::workspaces::remove_workspace(&db, id, guard.get_user()).await?;

    // Remove the workspace from the cache
    cache::workspaces::remove_workspace_cache(redis, workspace.id).await;

    // Return success
    Ok(ApiResponse::success(
        format!("Workspace {} deleted", workspace.id),
        None,
    ))
}
