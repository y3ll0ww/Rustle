use rocket::{http::CookieJar, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    database::{self, Db},
    policies::Policy,
    routes::projects::get_workspace_and_project,
};

#[delete("/<id>/delete")]
pub async fn delete_project_by_id(
    id: Uuid,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    let user = guard.get_user();

    // Get the workspace ID needed to validate the policy
    let workspace_id = get_workspace_and_project(id, &db, redis)
        .await
        .map(|(w, _)| w.workspace.id)?;

    // Run the policy to remove a project
    Policy::projects_remove(workspace_id, user, cookies)?;

    // Remove the project from the database (relevant records in different tables will be cascaded by Postgres)
    let project = database::projects::remove_project(&db, id).await?;

    // Remove the project from the cache
    cache::projects::remove_project_cache(redis, project.id).await;

    // Return success
    Ok(ApiResponse::success(
        format!("Project '{}' deleted", project.name),
        None,
    ))
}
