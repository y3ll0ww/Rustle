use super::*;

/// Deletes a [`Workspace`] and related [`WorkspaceMember`]s.
///
/// The table [`workspace_members::table`] is linked to [`workspaces::table`] and are cascaded upon
/// deletion.
///
/// ## Permissions
/// Request user ID must be the same as [`Workspace::owner`].
///
/// ## Request
/// * Method: `DELETE`
/// * Guarded by JWT token
/// * Data: `id: String`
/// * Database access
/// * Cookies: [`WORKSPACE_COOKIE`]
/// * Cache: [`team_cache_key`] with [`TEAM_CACHE_TTL`]
///
/// ## Response
/// * **200 OK**: Nothing returned.
/// * **401 Unauthorized**:
///   - No [`TOKEN_COOKIE`](crate::cookies::TOKEN_COOKIE).
///   - Request user is not the [`owner`](Workspace::owner) of the workspace.
/// * **404 Not found**: No [`Workspace`] found in [`workspaces::table`].
/// * **500 Server Error**: Any database operation fails.
pub async fn delete_workspace_by_id(
    id: Uuid,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    // Get user cookie
    let user_id = guard.get_user().id;

    // 1. Get the workspace from database
    let workspace_id = id;
    let workspace = db
        .run(move |conn| {
            workspaces::table
                .filter(workspaces::id.eq(&workspace_id))
                .first::<Workspace>(conn)
                .map_err(ApiResponse::from_error)
        })
        .await?;

    // 2. Verify the owner
    if user_id != workspace.owner {
        return Err(ApiResponse::unauthorized(format!(
            "User '{user_id}' not the owner",
        )));
    }

    // 3. Remove the relevant cookie
    remove_workspace_update_cookie(id, cookies);

    // 4. Remove the workspace information in the cache; if this fails ignore
    let _ = redis
        .lock()
        .await
        .remove_from_cache(&cache_key_workspace(id))
        .await;

    // 5. Remove the workspace from the database
    //    - Delete on cascade for workspace_members table
    let deleted_rows = db
        .run(move |conn| {
            diesel::delete(workspaces::table.filter(workspaces::id.eq(&id))).execute(conn)
        })
        .await
        .map_err(ApiResponse::from_error)?;

    if deleted_rows == 0 {
        return Err(ApiResponse::internal_server_error(
            "Nothing deleted".to_string(),
        ));
    }

    // Return success
    Ok(ApiResponse::success(
        format!("Workspace {} deleted", workspace.id),
        None,
    ))
}
