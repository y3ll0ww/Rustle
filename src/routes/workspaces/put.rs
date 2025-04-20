use rocket::{http::CookieJar, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    database::{self, Db},
    models::workspaces::{Workspace, WorkspaceUpdate}, policies::Policy,
};

#[put("/update/<id>", format = "json", data = "<update>")]
pub async fn update_workspace(
    id: Uuid,
    update: Json<WorkspaceUpdate>,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Workspace>, Error<Null>> {
    // Check if the user is authorized to perform this action
    Policy::update_workspaces_info(id, guard.get_user(), cookies)?;

    // Update the workspace information in the database
    let updated_workspace =
        database::workspaces::update_workspace_information(&db, id, update.clone().into_inner())
            .await?;

    // Update the workspace in the cache
    cache::workspaces::update_workspace_cache(redis, id, Some(update.into_inner()), None).await;

    // Return a success response
    Ok(ApiResponse::success(
        "Workspace updated successfully".to_string(),
        Some(updated_workspace),
    ))
}
