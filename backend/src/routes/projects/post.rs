use rocket::{http::CookieJar, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    database::{
        self,
        pagination::{records::PaginatedRecords, request::PaginationRequest, sort::ProjectField},
        Db,
    },
    models::{
        projects::{Project, ProjectMember, ProjectWithMembers},
        users::UserRole,
    },
    policies::Policy,
    routes::workspaces::get_workspace_with_members,
};

#[post("/?<workspace>&<user>", format = "json", data = "<params>")]
pub async fn get_paginated_projects(
    workspace: Option<Uuid>,
    user: Option<Uuid>,
    params: Json<PaginationRequest<ProjectField>>,
    guard: JwtGuard,
    db: Db,
    redis: &State<RedisMutex>,
) -> Result<Success<PaginatedRecords<Project>>, Error<Null>> {
    let auth_user = guard.get_user();

    // If no filters are applied and the user is not admin set self as user ID
    let user = match (workspace, user, auth_user.role) {
        (None, None, role) if role != i16::from(UserRole::Admin) => Some(auth_user.id),
        _ => user,
    };

    // Return not found if user is not a member of provided workspace
    if let Some(id) = workspace {
        // Get the workspace information with its members
        let workspace_with_members = get_workspace_with_members(id, &db, redis).await?;
        Policy::workspaces_view(&auth_user, &workspace_with_members)?;
    };

    // Return not found if user is not admin, self or part of a shared workspace
    if let Some(id) = user {
        Policy::users_get(&db, &auth_user, id).await?;
    }

    // Return the requested paginated result
    let page = database::projects::get_projects_paginated(&db, workspace, user, params).await?;

    Ok(ApiResponse::success(
        format!(
            "{} of {} projects shown",
            page.records_on_page(),
            page.total_records(),
        ),
        Some(page),
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
