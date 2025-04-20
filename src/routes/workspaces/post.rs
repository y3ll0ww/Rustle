use rocket::{form::Form, http::CookieJar, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    cookies,
    database::{self, Db},
    forms::workspace::NewWorkspaceForm,
    models::workspaces::{NewWorkspace, WorkspaceMember, WorkspaceRole, WorkspaceWithMembers},
    policies::Policy,
};

#[post("/new", data = "<form>")]
pub async fn create_new_workspace_by_form(
    form: Form<NewWorkspaceForm>,
    guard: JwtGuard,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
    db: Db,
) -> Result<Success<WorkspaceWithMembers>, Error<Null>> {
    // Get user information from cookies
    let user = guard.get_user();

    // Validate user permissions
    Policy::create_workspaces(&user)?;

    // Insert and return a new workspace with members
    let workspace_with_members = database::workspaces::insert_new_workspace(
        &db,
        NewWorkspace::from_form(user.id, form.into_inner()),
    )
    .await?;

    // Add the workspace information to the cache
    cache::workspaces::add_workspace_cache(redis, &workspace_with_members).await;

    // Add the workspace permission to cookies
    cookies::workspaces::insert_workspace_permission(
        cookies,
        workspace_with_members.workspace.id,
        i16::from(WorkspaceRole::Owner),
    )?;

    // Return success response
    Ok(ApiResponse::success(
        format!(
            "Workspace created: '{}'",
            workspace_with_members.workspace.name
        ),
        Some(workspace_with_members),
    ))
}

#[put("/add-members/<id>", format = "json", data = "<members>")]
pub async fn add_members_to_workspace(
    id: Uuid,
    members: Json<Vec<WorkspaceMember>>,
    guard: JwtGuard,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
    db: Db,
) -> Result<Success<WorkspaceWithMembers>, Error<Null>> {
    // Only allow this function if the user is admin or the workspace permissions are sufficient.
    Policy::update_workspaces_members(id, guard.get_user(), cookies)?;

    // Cannot add an empty vector
    if members.is_empty() {
        return Err(ApiResponse::bad_request("No members to add".to_string()));
    }

    // Cannot add a second owner
    if let Some(_) = members
        .iter()
        .find(|m| m.role == i16::from(WorkspaceRole::Owner))
    {
        return Err(ApiResponse::bad_request(
            "Cannot add another owner".to_string(),
        ));
    }

    // Extract the members length before going out of scope
    let members_len = members.len();

    // Add members to the workspace in the database
    let workspace_with_members =
        database::workspaces::add_members_to_workspace(&db, members.into_inner()).await?;

    // Update the workspace information in the cache
    cache::workspaces::add_workspace_cache(redis, &workspace_with_members).await;

    // Return success response
    Ok(ApiResponse::success(
        format!(
            "{members_len} members added to '{}'",
            workspace_with_members.workspace.name
        ),
        Some(workspace_with_members),
    ))
}
