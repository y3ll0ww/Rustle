use rocket::{http::CookieJar, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    cookies,
    database::{self, Db}, models::workspaces::{Workspace, WorkspaceUpdate},
};

#[put("/<id>/update", format = "json", data = "<update>")]
pub async fn update_workspace(
    id: Uuid,
    update: Json<WorkspaceUpdate>,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Workspace>, Error<Null>> {
    // Get user information from cookies
    let user = guard.get_user();

    // Update the workspace information in the database
    let updated_workspace = database::workspaces::update_workspace_information(
        &db,
        id,
        user,
        update.clone().into_inner(),
    )
    .await?;

    // Update the workspace in the cache
    cache::workspaces::update_workspace_cache(redis, id, Some(update.into_inner()), None).await;

    // Add the latest update in the cookies
    cookies::workspaces::add_workspace_timestamp(
        updated_workspace.id,
        updated_workspace.updated_at,
        cookies,
    );

    // Return a success response
    Ok(ApiResponse::success(
        "Workspace updated successfully".to_string(),
        Some(updated_workspace),
    ))
}
