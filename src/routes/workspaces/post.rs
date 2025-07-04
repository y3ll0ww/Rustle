use std::collections::HashSet;

use rocket::{form::Form, http::CookieJar, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    cookies,
    database::{self, Db},
    email::MailClient,
    forms::{
        invite::InvitedMultipleUsersForm, projects::NewProjectForm, workspace::NewWorkspaceForm,
    },
    models::{
        projects::{NewProject, ProjectRole, ProjectWithMembers},
        users::{InvitedUser, PublicUser, UserStatus},
        workspaces::{NewWorkspace, WorkspaceMember, WorkspaceRole, WorkspaceWithMembers},
    },
    policies::Policy,
};

const MAX_SIMILAR_USERNAMES: usize = 100;

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
    Policy::workspaces_create(&user)?;

    // Insert and return a new workspace with members
    let workspace_with_members = database::workspaces::insert_new_workspace(
        &db,
        NewWorkspace::from_form(user.id, form.into_inner()),
    )
    .await?;

    // Add the workspace information to the cache
    cache::workspaces::add_workspace_cache(redis, &workspace_with_members).await;

    // Add the workspace permission to cookies
    cookies::permissions::insert_workspace_permission(
        workspace_with_members.workspace.id,
        i16::from(WorkspaceRole::Owner),
        cookies,
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

#[post("/<id>/add-members", format = "json", data = "<members>")]
pub async fn add_members_to_workspace(
    id: Uuid,
    members: Json<Vec<WorkspaceMember>>,
    guard: JwtGuard,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
    db: Db,
) -> Result<Success<WorkspaceWithMembers>, Error<Null>> {
    // Only allow this function if the user is admin or the workspace permissions are sufficient.
    Policy::workspaces_update_members(id, guard.get_user(), cookies)?;

    // Cannot add an empty vector
    if members.is_empty() {
        return Err(ApiResponse::bad_request("No members to add".to_string()));
    }

    // Cannot add a second owner
    if members
        .iter()
        .any(|m| m.role == i16::from(WorkspaceRole::Owner))
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

#[post("/<id>/invite", data = "<form>")]
pub async fn invite_new_users_to_workspace(
    id: Uuid,
    guard: JwtGuard,
    form: Form<InvitedMultipleUsersForm<'_>>,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Vec<String>>, Error<Null>> {
    // Only allow this function if the user is admin or the workspace permissions are sufficient.
    Policy::workspaces_update_members(id, guard.get_user(), cookies)?;

    // Create a vector of Users and a HashSet of base usernames from the form
    let (mut invited_users, base_usernames) = form
        .get_users_and_base_usernames()
        .map_err(ApiResponse::internal_server_error)?;

    // Collect duplicate usernames from the database
    let mut existing_usernames =
        database::users::get_username_duplicates(&db, &base_usernames).await?;

    // Update the usernames of the new users to avoid unique constraint violations
    assign_unique_usernames(&mut invited_users, &mut existing_usernames)
        .map_err(ApiResponse::bad_request)?;

    // Insert the new users into the database in a single transaction
    let inserted_users =
        database::workspaces::create_transaction_bulk_invitation(&db, id, invited_users).await?;

    // Declare a vector to keep the tokens
    let mut tokens = Vec::new();

    // Loop through the collection of new users
    for user in &inserted_users {
        // Create a random token with a length of 64 characters
        let token = cache::create_random_token(64);

        // Save the token for the response
        tokens.push(token.clone());

        // Add the token to the redis cache; containing the user ID
        cache::users::add_invite_token(redis, &token, user.id).await?;

        let inviter = guard.get_user();
        let recipient = PublicUser::from(user);
        let workspace_name = get_workspace_name(id, &db, redis).await?;

        // Send an invitation email to the new users, containing the token
        tokio::task::spawn_blocking(move || {
            let _ = MailClient::no_reply().send_invitation(
                &inviter,
                &recipient,
                &workspace_name,
                &token,
            );
        });
    }

    // Return success response
    Ok(ApiResponse::success(
        format!("{} users invited", inserted_users.len()),
        Some(tokens),
    ))
}

/// TODO!:
/// - Adding space/project functionality
/// - Inviting only when a certain role in space
#[post("/<id>/re-invite/<member>")]
pub async fn reinvite_user_by_id(
    id: Uuid,
    member: Uuid,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<String>, Error<Null>> {
    // Only allow this function if the user is admin or the workspace permissions are sufficient.
    Policy::workspaces_update_members(id, guard.get_user(), cookies)?;

    // Get the user from the database
    let user = database::users::get_user_by_id(&db, member).await?;

    // Extract the user status
    let user_status =
        UserStatus::try_from(user.status).map_err(|e| ApiResponse::conflict(e, String::new()))?;

    // Make sure the user status is still on invited
    if !matches!(user_status, UserStatus::Invited) {
        return Err(ApiResponse::bad_request(format!(
            "User {} has status {user_status:?}",
            user.username,
        )));
    };

    // Create a random token with a length of 64 characters
    let token = cache::create_random_token(64);

    // Add the token to the redis cache; containing the user ID
    cache::users::add_invite_token(redis, &token, user.id).await?;

    // Get the required information for the invitation email
    let inviter = guard.get_user();
    let recipient = PublicUser::from(&user);
    let workspace_name = get_workspace_name(id, &db, redis).await?;

    // Send the email
    MailClient::no_reply()
        .send_invitation(&inviter, &recipient, &workspace_name, &token)
        .map_err(ApiResponse::internal_server_error)?;

    Ok(ApiResponse::success(
        format!("{} invited", user.username),
        Some(token),
    ))
}

async fn get_workspace_name(
    id: Uuid,
    db: &Db,
    redis: &State<RedisMutex>,
) -> Result<String, Error<Null>> {
    // Try to retrieve the workspace name from the cache or the database
    let name = match cache::workspaces::get_workspace_cache(redis, id).await {
        Ok(Some(workspace_with_members)) => workspace_with_members.workspace.name,
        _ => database::workspaces::get_workspace_by_id(db, id)
            .await
            .map(|workspace_with_members| workspace_with_members.workspace.name)?,
    };

    Ok(name)
}

fn assign_unique_usernames(
    new_users: &mut [InvitedUser],
    existing_usernames: &mut HashSet<String>,
) -> Result<(), String> {
    // Loop through the new users and check if their usernames are already taken
    for user in new_users.iter_mut() {
        let mut suffix = 1;
        let mut assigned_username = user.username.clone();

        // If the username is already taken, append a suffix
        while existing_usernames.contains(&assigned_username) {
            assigned_username = format!("{}_{}", user.username, suffix);
            suffix += 1;

            // If the suffix is greater than the maximum, return an error
            if suffix > MAX_SIMILAR_USERNAMES {
                return Err(format!("Too many usernames containing '{}'", user.username));
            }
        }

        // Add the unique username to the existing usernames set
        existing_usernames.insert(assigned_username.clone());

        // Update the username with the unique username
        user.username = assigned_username;
    }

    Ok(())
}

#[post("/<id>/new_project", data = "<form>")]
pub async fn create_new_project_by_form(
    id: Uuid,
    form: Form<NewProjectForm>,
    guard: JwtGuard,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
    db: Db,
) -> Result<Success<ProjectWithMembers>, Error<Null>> {
    // Validate user permissions
    Policy::projects_create(id, guard.get_user(), cookies)?;

    // Extract the important information from the form
    let new_project = NewProject::from_form(form.into_inner());

    // Create a new project (without members)
    let project_with_members = database::projects::insert_new_project(&db, id, new_project).await?;

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
