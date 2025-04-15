use rocket::State;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    database::{users as database, Db},
    models::users::{PublicUser, UserStatus},
};

#[get("/")]
pub async fn list_all_users(
    _guard: JwtGuard,
    db: Db,
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
    db: Db,
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

#[get("/invite/get/<token>")]
pub async fn get_invited_user(
    token: String,
    db: Db,
    redis: &State<RedisMutex>,
) -> Result<Success<Vec<String>>, Error<Null>> {
    // Get the user ID from the cache (should be a UUID at this stage)
    let user_id = cache::users::get_invite_token(redis, &token).await?;

    // Get the user from the database
    let user = database::get_user_by_id(&db, user_id).await?;

    // Return not found if the user is not of status invited
    // > Returning not found avoids leaking user existence or status, preventing malicious actors
    // > from probing valid invitation tokens.
    if user.status != i16::from(UserStatus::Invited) {
        return Err(ApiResponse::not_found(format!(
            "User '{user_id}' not found",
        )));
    }

    // Return success response
    Ok(ApiResponse::success("User set in cache".to_string(), None))
}
