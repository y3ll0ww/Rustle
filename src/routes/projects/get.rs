use rocket::{http::CookieJar, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    cookies,
    database::{self, Db},
    models::projects::{Project, ProjectWithMembers},
    policies::Policy,
    routes::workspaces::get_workspace_with_members,
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

/// Returns information about a workspace, including its members.
#[get("/<id>")]
pub async fn get_project_by_id(
    id: Uuid,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<ProjectWithMembers>, Error<Null>> {
    let user = guard.get_user();

    // Get the project with members
    let project_with_members = get_project_with_members(id, &db, redis).await?;

    // Get the workspace information with its members
    let workspace_with_members =
        get_workspace_with_members(project_with_members.project.workspace, &db, redis).await?;

    // Run the policy to view a project
    Policy::projects_view(&user, &workspace_with_members)?;

    // Insert the project permissions in cookies
    if let Some(member) = project_with_members
        .members
        .iter()
        .find(|m| m.user.id == user.id)
    {
        cookies::permissions::insert_project_permission(id, member.role, cookies)?;
    }

    Ok(ApiResponse::success(
        format!(
            "Project '{}' from database",
            project_with_members.project.name,
        ),
        Some(project_with_members),
    ))
}

async fn get_project_with_members(
    id: Uuid,
    db: &Db,
    redis: &State<RedisMutex>,
) -> Result<ProjectWithMembers, Error<Null>> {
    Ok(match cache::projects::get_project_cache(redis, id).await? {
        Some(cached_project) => cached_project,
        None => {
            // Get the project with members from the database
            let project_from_database = database::projects::get_project_by_id(&db, id).await?;

            // Add the project with members to the cache
            cache::projects::add_project_cache(redis, &project_from_database).await;

            // Return a fresh project with members from the database
            project_from_database
        }
    })
}
