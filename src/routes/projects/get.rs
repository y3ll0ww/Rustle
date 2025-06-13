use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    database::{self, Db},
    models::projects::Project,
};

/// Returns an overview of workspaces of which the request user is a member.
#[get("/")]
pub async fn get_projects_of_current_user(
    guard: JwtGuard,
    db: Db,
) -> Result<Success<Vec<Project>>, Error<Null>> {
    let user = guard.get_user();

    // Retrieve all workspaces with the user ID
    let projects = database::projects::get_projects_by_user_id(&db, user.id).await?;

    // Return vector of workspaces
    Ok(ApiResponse::success(
        format!("Projects for '{}'", user.username),
        Some(projects),
    ))
}
