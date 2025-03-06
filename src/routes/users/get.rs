use super::*;

#[get("/")]
pub async fn list_all_users(db: Database) -> Result<Success<Vec<User>>, Error<Null>> {
    db.run(move |conn| {
        users::table.get_results::<User>(conn)
    })
    .await
    .map(|user| ApiResponse::success("Returning all users".to_string(), Some(user)))
    .map_err(|e| ApiResponse::not_found(e.to_string()))
}

#[get("/<username>")]
pub async fn get_user_by_username(
    username: String,
    db: Database,
) -> Result<Success<User>, Error<Null>> {
    // Only get the user from the database
    get_user_from_db(db, username).await
}
