use rocket::State;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    database::{users as database, Db},
    models::users::{PublicUser, UserRole, UserStatus},
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

#[derive(Deserialize, Serialize)]
pub struct UserPagination {
    last_id: Option<Uuid>,
    users: Vec<PublicUser>,
}

#[get("/?<after>&<limit>")]
pub async fn get_all_users_paginated(
    after: Option<Uuid>,
    limit: Option<i64>,
    guard: JwtGuard,
    db: Db,
) -> Result<Success<UserPagination>, Error<Null>> {
    // Only admin can see all the users
    if guard.get_user().role != i16::from(UserRole::Admin) {
        return Err(ApiResponse::unauthorized(format!(
            "Only admin can see all users"
        )));
    }
    
    let users = database::get_users_after_id(&db, after, limit.unwrap_or(20)).await?;
    let users_len = users.len();
    let last_id = users.last().map(|user| user.id);

    let pagination = UserPagination { last_id, users };

    Ok(ApiResponse::success(
        format!("{users_len} users found"),
        Some(pagination),
    ))
}

#[get("/<username>")]
pub async fn get_user_by_username(
    username: &str,
    _guard: JwtGuard,
    db: Db,
) -> Result<Success<PublicUser>, Error<Null>> {
    // Only get the user from the database
    database::get_user_by_username(&db, username)
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
    token: &str,
    db: Db,
    redis: &State<RedisMutex>,
) -> Result<Success<Vec<String>>, Error<Null>> {
    // Get the user ID from the cache (should be a UUID at this stage)
    let user_id = cache::users::get_invite_token(redis, token).await?;

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
