use super::*;

/// Deletes a [`User`] from the database.
///
/// ## Permissions
/// - Request user has [`UserRole::Admin`]
/// - OR request user ID is the same as the user ID defined in the request
///
/// ## Request
/// * Method: `DELETE`
/// * Data: `id: String`
/// * Guarded by JWT token
/// * Database access
///
/// ## Response
/// * **200 OK**: Nothing returned.
/// * **401 Unauthorized**:
///   - No [`TOKEN_COOKIE`]
///   - Request user is not [`UserRole::Admin`] (optional).
///   - Request user not the same user (optional).
/// * **404 Not found**: No [`User`] found in [`users::table`].
/// * **500 Server Error**: Any database operation fails.
pub async fn delete_user_by_id(
    id: Uuid,
    guard: JwtGuard,
    db: Database,
) -> Result<Success<Null>, Error<Null>> {
    // Get user cookie
    let user = guard.get_user();

    // Return early if the user to delete is not self or admin
    if user.role != UserRole::Admin && user.id != id {
        return Err(ApiResponse::unauthorized(
            "No permission to delete user".to_string(),
        ));
    }

    // Define the response messages beforehand
    let success_msg = format!("User with ID '{id}' successfully deleted");
    let failed_msg = format!("User with ID '{id}' not found");

    // Delete the records from the database and collect the number of deleted rows
    let deleted_rows = db
        .run(move |conn| diesel::delete(users::table.filter(users::id.eq(id))).execute(conn))
        .await
        .map_err(ApiResponse::from_error)?;

    // If there are any deleted rows, it means the user is successfully deleted
    if deleted_rows > 0 {
        Ok(ApiResponse::success(success_msg, None))
    } else {
        Err(ApiResponse::not_found(failed_msg))
    }
}
