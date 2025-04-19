use chrono::NaiveDateTime;

use crate::models::users::{PublicUser, User};

use super::*;

/// Returns an overview of workspaces of which the request user is a member.
///
/// ## Request
/// * Method: `GET`
/// * Guarded by JWT token
/// * Database access
///
/// ## Response
/// * **200 Created**: Returns a vector of [`Workspace`]s.
/// * **401 Unauthorized**: No [`TOKEN_COOKIE`](crate::cookies::TOKEN_COOKIE).
/// * **500 Server Error**: Any database operation fails.
pub async fn get_workspaces_by_user_id(
    guard: JwtGuard,
    db: Db,
) -> Result<Success<Vec<Workspace>>, Error<Null>> {
    let user_id = guard.get_user().id;

    // Set the success message
    let success_message = format!("Workspaces for user '{user_id}'");

    // Retrieve all workspaces with the user ID
    let workspaces = db
        .run(move |conn| {
            workspaces::table
                .inner_join(
                    workspace_members::table.on(workspace_members::workspace.eq(workspaces::id)),
                )
                .filter(workspace_members::member.eq(user_id))
                .select(workspaces::all_columns)
                .load::<Workspace>(conn)
        })
        .await
        .map_err(ApiResponse::from_error)?;

    // Return vector of workspaces
    Ok(ApiResponse::success(success_message, Some(workspaces)))
}

/// Returns information about a workspace, including its members.
///
/// ## Request
/// * Method: `GET`
/// * Guarded by JWT token
/// * Database access
/// * Cookies: [`USER_COOKIE`](crate::cookies::USER_COOKIE)
/// * Cache: [`team_cache_key`] with [`TEAM_CACHE_TTL`]
///
/// ## Response
/// * **200 Created**: Returns [`WorkspaceWithMembers`].
///   - If there is a [`TeamUpdate`] in the cookie and the timestamp is the same as the timestamp
///     retrieved from the database, it returns from the **cache** (if it exists).
///   - If not; it retrieves from the database, adds the [`TeamUpdate`] to the cookies and the
///     [`TeamWithMembers`] to the cache (which it also returns in the response).
/// * **401 Unauthorized**: No [`TOKEN_COOKIE`](crate::cookies::TOKEN_COOKIE).
/// * **404 Not found**: No [`TeamUpdate`], [`TeamMember`]s or [`Team`] in the database.
/// * **500 Server Error**: Any database operation fails.
pub async fn get_workspace_by_id(
    id: Uuid,
    _guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<WorkspaceWithMembers>, Error<Null>> {
    let redis = redis.lock().await;

    // Step 1: Check the workspace update stored in cookies
    let timestamp_from_cookie = get_workspace_timestamp(id, cookies).unwrap_or_default();

    // Step 2: Get the workspace update from the database
    let workspace_id = id;
    let updated_at = db
        .run(move |conn| {
            workspaces::table
                .select(workspaces::updated_at)
                .filter(workspaces::id.eq(&workspace_id))
                .first::<NaiveDateTime>(conn)
                .map_err(ApiResponse::from_error)
        })
        .await?;

    // If there is a workspace update in the cookie and the timestamp is the same as the timestamp
    // retrieved from the database, we can return the workspace from the cache (if it exists).
    if timestamp_from_cookie == updated_at {
        let workspace_from_cache = redis
            .get_from_cache::<WorkspaceWithMembers>(&cache_key_workspace(id))
            .await
            .unwrap_or(None);

        // If there is workspace information in the cache, return it
        if let Some(workspace_with_members) = workspace_from_cache.as_ref() {
            return Ok(ApiResponse::success(
                format!(
                    "Workspace '{}' from cache",
                    workspace_with_members.workspace.name
                ),
                workspace_from_cache,
            ));
        }
    }

    // In any other case (no cookie or different timestamps); update the cookie
    add_workspace_timestamp(workspace_id, updated_at, cookies);

    // Step 4: Get the workspace information including workspace members from the database
    //let workspace_id = id;
    let workspace_from_database = db
        .run(move |conn| {
            let workspace = workspaces::table
                .filter(workspaces::id.eq(&id))
                .first::<Workspace>(conn)
                .map_err(ApiResponse::from_error)?;

            let members = workspace_members::table
                .inner_join(users::table.on(users::id.eq(workspace_members::member)))
                .filter(workspace_members::workspace.eq(&workspace_id))
                .select((users::all_columns, workspace_members::role))
                .load::<(User, i16)>(conn)
                .map_err(ApiResponse::from_error)?
                .into_iter()
                .map(|(user, role)| MemberInfo {
                    user: PublicUser::from(&user),
                    role,
                })
                .collect();

            Ok(WorkspaceWithMembers { workspace, members })
        })
        .await?;

    // Add the workspace with members to the cache (ignore error in case cache not working)
    let _ = redis
        .set_to_cache(
            &cache_key_workspace(id),
            &workspace_from_database,
            CACHE_TTL_ONE_HOUR,
        )
        .await;

    Ok(ApiResponse::success(
        format!(
            "Workspace '{}' from database",
            workspace_from_database.workspace.name
        ),
        Some(workspace_from_database),
    ))
}
