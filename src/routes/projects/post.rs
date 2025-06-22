use rocket::{form::Form, http::CookieJar, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    cookies,
    database::{self, Db},
    forms::projects::NewProjectForm,
    models::projects::{NewProject, ProjectMember, ProjectRole, ProjectWithMembers},
    policies::Policy,
};

#[post("/<workspace>/new", data = "<form>")]
pub async fn create_new_project_by_form(
    workspace: Uuid,
    form: Form<NewProjectForm>,
    guard: JwtGuard,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
    db: Db,
) -> Result<Success<ProjectWithMembers>, Error<Null>> {
    // Validate user permissions
    Policy::projects_create(workspace, guard.get_user(), cookies)?;

    // Extract the important information from the form
    let new_project = NewProject::from_form(form.into_inner());

    // Create a new project (without members)
    let project_with_members =
        database::projects::insert_new_project(&db, workspace, new_project).await?;

    // Add the project information to the cache
    cache::projects::add_project_cache(redis, &project_with_members).await;

    // Add the project permission to cookies
    cookies::permissions::insert_project_permission(
        project_with_members.project.id,
        i16::from(ProjectRole::Owner),
        cookies,
    )?;

    // Return success response
    Ok(ApiResponse::success(
        format!("Project created: '{}'", project_with_members.project.name),
        Some(project_with_members),
    ))
}

#[post("/<id>/add-members", format = "json", data = "<members>")]
pub async fn add_members_to_project(
    id: Uuid,
    members: Json<Vec<ProjectMember>>,
    guard: JwtGuard,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
    db: Db,
) -> Result<Success<ProjectWithMembers>, Error<Null>> {
    // Only allow this function if the user is admin or the project permissions are sufficient.
    Policy::project_update_members(id, guard.get_user(), cookies)?;

    // Cannot add an empty vector
    if members.is_empty() {
        return Err(ApiResponse::bad_request("No members to add".to_string()));
    }

    // Extract the members length before going out of scope
    let members_len = members.len();

    // Add members to the project in the database
    let project_with_members =
        database::projects::add_members_to_project(&db, members.into_inner()).await?;

    // Update the project information in the cache
    cache::projects::add_project_cache(redis, &project_with_members).await;

    // Return success response
    Ok(ApiResponse::success(
        format!(
            "{members_len} members added to '{}'",
            project_with_members.project.name
        ),
        Some(project_with_members),
    ))
}
