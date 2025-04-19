//use chrono::Utc;
//
//use crate::{
//    cache::workspaces::{set_team_cache, update_team_cache},
//    forms::teams::UpdateTeamForm,
//};
//
use super::*;

/// Creates a new team.
///
/// ## Permissions
/// Requires as least [`UserRole::Manager`] to successfully complete.
///
/// ## Request
/// * Method: `POST`
/// * Guarded by JWT token
/// * Data: [`NewTeamForm`]
/// * Database access
/// * Cookies: [`TEAM_COOKIE`]
/// * Cache: [`team_cache_key`] with [`TEAM_CACHE_TTL`]
///
/// ## Response
/// * **201 Created**: Nothing returned.
///   - [`Team`] added to [`teams::table`] with request user as `owner_id`.
///   - Request user (from cookie) added as [`TeamMember`] with [`TeamRole::Owner`] to
///     [`team_members::table`].
///   - [`TeamUpdate`] added to [`team_updates::table`] with `updated_at`.
///   - [`TeamUpdate`] is added as a cookie.
///   - [`TeamWithMembers`] added to the **Redis** cache (ignore if fail).
/// * **401 Unauthorized**:
///   - No [`TOKEN_COOKIE`](crate::cookies::TOKEN_COOKIE).
///   - Request user is not [`UserRole::Manager`] or higher.
/// * **500 Server Error**: Any database operation fails.
pub async fn create_new_workspace_by_form(
    form: Form<NewWorkspaceForm>,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    // Get user information from cookies
    let user = guard.get_user();

    // Step 1: Validate user permissions
    if user.role < i16::from(UserRole::Manager) {
        return Err(ApiResponse::unauthorized(
            "User not allowed to create teams".to_string(),
        ));
    }

    // Create a new Team
    let new_workspace = Workspace::new(user.id, form.name.clone(), form.description.clone());

    // Create a new team member
    let owner_membership = WorkspaceMember {
        workspace: new_workspace.id,
        member: user.id,
        role: WorkspaceRole::Owner as i16,
    };

    // Create some variables from which types will go out of scope
    let success_message = format!("Team created: '{}'", form.name);
    let team_id = new_workspace.id;
    let user = user.clone();

    let workspace_id = new_workspace.id;
    let timestamp = new_workspace.updated_at;

    // Step 2: Add new team to database
    let cache_team_with_members = db
        .run(move |conn| {
            // Create a new team with members to place in the cache
            let team_with_members = WorkspaceWithMembers {
                workspace: new_workspace.clone(),
                members: vec![MemberInfo {
                    user,
                    role: owner_membership.role,
                }],
            };

            // Insert new team into teams table
            diesel::insert_into(workspaces::table)
                .values(&new_workspace)
                .execute(conn)?;

            // Insert owner into team_members table
            diesel::insert_into(workspace_members::table)
                .values(&owner_membership)
                .execute(conn)?;

            // Return the team with members for caching
            Ok(team_with_members)
        })
        .await
        .map_err(ApiResponse::from_error)?;

    // Step 3: Add the team update to the cookies
    add_workspace_update_cookie(workspace_id, timestamp, cookies);

    // Step 4: Add the team information to the cache
    set_workspace_cache(redis, team_id, &cache_team_with_members).await;

    // Return success response
    Ok(ApiResponse::success(success_message, None))
}

pub async fn update_workspace_by_form(
    id: Uuid,
    form: Form<UpdateWorkspaceForm>,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    // User must be a manager
    let minimal_user_role = UserRole::Manager;
    let minimal_workspace_role = WorkspaceRole::Contributor;

    // Get user information from cookies
    let user = guard.get_user();

    // Copy some values to prevent borrowing issues
    let workspace_id = id;
    let form_clone = form.clone();

    // Perform database actions
    let timestamp = db
        .run(move |conn| {
            // Check if the user has the correct user role
            if user.role < i16::from(minimal_user_role) {
                // If the user doesn't have the right role; check the role within the workspace
                let member = workspace_members::table
                    .filter(workspace_members::workspace.eq(&workspace_id))
                    .filter(workspace_members::member.eq(&user.id))
                    .first::<WorkspaceMember>(conn)
                    .map_err(ApiResponse::from_error)?;

                // If the workspace role is (also) insufficient; return unauthorized
                if member.role < i16::from(minimal_workspace_role) {
                    return Err(ApiResponse::unauthorized(
                        "No permission to update team information".to_string(),
                    ));
                }
            }

            // Set the current timestamp
            let timestamp = Utc::now().naive_utc();

            // Step 3: Update the workspace table with information from the form
            if diesel::update(workspaces::table.filter(workspaces::id.eq(&workspace_id)))
                .set((&*form, workspaces::updated_at.eq(timestamp)))
                .execute(conn)
                .map_err(ApiResponse::from_error)?
                == 0
            {
                return Err(ApiResponse::not_found("No workspace to update".to_string()));
            };

            Ok(timestamp)
        })
        .await?;

    // Step 5: Update the team in the cache
    if update_workspace_cache(redis, id, Some(form_clone), None).await {
        // Step 6: If cache has been updated, update the cookie too
        add_workspace_update_cookie(workspace_id, timestamp, cookies);
    }

    // Return a success response
    Ok(ApiResponse::success(
        "Workspace updated successfully".to_string(),
        None,
    ))
}

//pub id: String,
//pub owner_id: String,
//pub team_name: String,
//pub team_description: Option<String>,
//pub image_url: Option<String>,
//pub created_at: NaiveDateTime,
//pub updated_at: NaiveDateTime,

// Change owner
// Add member
