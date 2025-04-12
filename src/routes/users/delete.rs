use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    database::{Db, users},
    models::users::UserRole,
};

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
#[delete("/<id>/delete")]
pub async fn delete_user_by_id(
    id: Uuid,
    guard: JwtGuard,
    db: Db,
) -> Result<Success<Null>, Error<Null>> {
    // Get user cookie
    let user = guard.get_user();

    // Return early if the user to delete is not self or admin
    if user.role != i16::from(UserRole::Admin) && user.id != id {
        return Err(ApiResponse::unauthorized(
            "No permission to delete user".to_string(),
        ));
    }

    // Delete the records from the database and collect the number of deleted rows
    let deleted_rows = users::delete_user_by_id(&db, id).await?;

    // If there are any deleted rows, it means the user is successfully deleted
    if deleted_rows > 0 {
        Ok(ApiResponse::success(format!("User '{id}' deleted"), None))
    } else {
        Err(ApiResponse::not_found(format!("User '{id}' not found")))
    }
}
