use rocket::{form::Form, http::CookieJar, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    cookies,
    database::{self, Db},
    forms::projects::NewProjectForm,
    models::projects::{NewProject, ProjectRole, ProjectWithMembers},
    policies::Policy,
};

#[post("/new/<workspace>", data = "<form>")]
pub async fn create_new_project_by_form(
    workspace: Uuid,
    form: Form<NewProjectForm>,
    guard: JwtGuard,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
    db: Db,
) -> Result<Success<ProjectWithMembers>, Error<Null>> {
    // Get user information from cookies
    let user = guard.get_user();

    // Validate user permissions
    Policy::projects_create(workspace, user, cookies)?;

    // Extract the important information from the form
    let new_project = NewProject::from_form(form.into_inner());

    // Create a new project (without members)
    let project_with_members = database::projects::insert_new_project(&db, new_project).await?;

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
