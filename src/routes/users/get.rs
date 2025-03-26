use super::*;

#[get("/")]
pub async fn list_all_users(
    _guard: JwtGuard,
    db: Database,
) -> Result<Success<Vec<PublicUser>>, Error<Null>> {
    let users: Vec<PublicUser> = db
        .run(move |conn| users::table.get_results::<User>(conn))
        .await
        .map_err(|e| ApiResponse::not_found(e.to_string()))?
        .iter()
        .map(PublicUser::from)
        .collect();

    //let public_users: Vec<PublicUser> = users.iter().map(|user| PublicUser::from(user)).collect();
    Ok(ApiResponse::success(
        format!("{} users found", users.len()),
        Some(users),
    ))
}

#[get("/<username>")]
pub async fn get_user_by_username(
    username: String,
    _guard: JwtGuard,
    db: Database,
) -> Result<Success<PublicUser>, Error<Null>> {
    // Only get the user from the database
    match &get_user_from_db(db, &username).await?.data {
        Some(data) => Ok(ApiResponse::success(
            format!("User '{username}'"),
            Some(PublicUser::from(data)),
        )),
        None => Err(ApiResponse::internal_server_error(format!(
            "User '{username}' found but unwrap failed."
        ))),
    }
}
