use rocket::{http::CookieJar, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    database::{self, Db},
    models::projects::{Project, ProjectUpdate},
    policies::Policy,
};

#[put("/<id>/update", format = "json", data = "<update>")]
pub async fn update_project(
    id: Uuid,
    update: Json<ProjectUpdate>,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Project>, Error<Null>> {
    // Check if the user is authorized to perform this action
    Policy::projects_update_info(id, guard.get_user(), cookies)?;

    // Update the project information in the database
    let updated_project =
        database::projects::update_project_information(&db, id, update.clone().into_inner())
            .await?;

    // Update the project in the cache
    cache::projects::update_project_cache(redis, id, &updated_project).await;

    // Return a success response
    Ok(ApiResponse::success(
        "Project updated successfully".to_string(),
        Some(updated_project),
    ))
}
