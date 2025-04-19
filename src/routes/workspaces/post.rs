use crate::{cache, cookies, database, models::workspaces::NewWorkspace};

use super::*;

pub async fn create_new_workspace_by_form(
    form: Form<NewWorkspaceForm>,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    // Get user information from cookies
    let user = guard.get_user();

    // Validate user permissions
    if user.role < i16::from(UserRole::Manager) {
        return Err(ApiResponse::unauthorized(
            "User not allowed to create teams".to_string(),
        ));
    }

    // Insert and return a new workspace with members
    let workspace_with_members = database::workspaces::insert_new_workspace(
        NewWorkspace::from_form(user.id, form.into_inner()),
        &db,
    )
    .await?;

    // Add the workspace update timestamp to the cookies
    cookies::workspaces::add_workspace_update_cookie(
        workspace_with_members.workspace.id,
        workspace_with_members.workspace.updated_at,
        cookies,
    );

    // Add the workspace information to the cache
    cache::workspaces::set_workspace_cache(redis, &workspace_with_members).await;

    // Return success response
    Ok(ApiResponse::success(
        format!(
            "Workspace created: '{}'",
            workspace_with_members.workspace.name
        ),
        None,
    ))
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

// Change owner
// Add member
