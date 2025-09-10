use rocket::{http::CookieJar, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::RedisMutex,
    cookies,
    database::{
        self,
        pagination::{records::PaginatedRecords, request::PaginationRequest, sort::ProjectField},
        Db,
    },
    models::{
        projects::{Project, ProjectWithMembers},
        users::UserRole,
    },
    policies::Policy,
    routes::{projects::get_workspace_and_project, workspaces::get_workspace_with_members},
};

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

    // Get the workspace and the project information
    let (workspace_with_members, project_with_members) =
        get_workspace_and_project(id, &db, redis).await?;

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

//Instead of get_paginated_users, maybe browse_users or list_users_paginated — to match REST semantics more intuitively.
#[get("/?<workspace>&<user>", format = "json", data = "<params>")]
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
