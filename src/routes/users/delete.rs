use super::*;

/// Deletes a [`User`] from the database.
///
/// **Route**: `./delete/<id>`
///
/// ### Parameters
/// * `db`: Instance of the [`Database`] connection.
/// * `id`: The ID from the [`User`] to be deleted.
///
/// ### Returns
/// * `Ok(Success<String>)`: When `Ok`, it returns a wrapped in [`Success`] with [`Null`] data.
/// * `Err(Error<String>)`: When `Err`, it returns an [`Error`] with [`Null`] data.
#[delete("/<id>/delete")]
pub async fn delete_user_by_id(id: String, db: Database) -> Result<Success<Null>, Error<Null>> {
    // Define the response messages beforehand
    let success_msg = format!("User with ID '{id}' successfully deleted");
    let failed_msg = format!("User with ID '{id}' not removed");

    // Delete the records from the database and collect the number of deleted rows
    let deleted_rows = db
        .run(move |conn| diesel::delete(users::table.filter(users::id.eq(id))).execute(conn))
        .await
        .map_err(|e| match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                ApiResponse::bad_request(format!("{}: {:?} - {:?}", failed_msg, kind, info))
            }
            other => ApiResponse::internal_server_error(format!("{}: {}", failed_msg, other)),
        })?;

    // If there are any deleted rows, it means the user is successfully deleted
    if deleted_rows > 0 {
        Ok(ApiResponse::success(success_msg, None))
    } else {
        Err(ApiResponse::not_found(format!(
            "{}: No user found with that ID",
            failed_msg
        )))
    }
}
