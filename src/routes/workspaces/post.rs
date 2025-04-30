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
    forms::{invite::InvitedMultipleUsersForm, workspace::NewWorkspaceForm},
    models::{
        users::{InvitedUser, PublicUser},
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
    Policy::update_workspaces_members(id, guard.get_user(), cookies)?;

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

    // Extract the workspace
    let workspace_name = match cache::workspaces::get_workspace_cache(redis, id).await {
        Ok(Some(workspace_with_members)) => workspace_with_members.workspace.name,
        _ => database::workspaces::get_workspace_by_id(&db, id)
            .await
            .map(|workspace_with_members| workspace_with_members.workspace.name)?,
    };

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
        let workspace_name = workspace_name.clone();

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
