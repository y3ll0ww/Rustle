use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    db::Database,
    models::users::PublicUser,
};

use super::*;

#[get("/")]
pub async fn list_all_users(
    _guard: JwtGuard,
    db: Database,
) -> Result<Success<Vec<PublicUser>>, Error<Null>> {
    let users = database::get_all_public_users(&db).await?;

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
    database::get_user_by_username(&db, &username)
        .await
        .map(|user| {
            ApiResponse::success(
                format!("User '{username}' found"),
                Some(PublicUser::from(&user)),
            )
        })
}
