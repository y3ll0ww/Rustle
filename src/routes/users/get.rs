use rocket::{serde::json::Json, State};

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    database::{
        self,
        pagination::{records::PaginatedRecords, request::PaginationRequest, sort::UserField},
        Db,
    },
    models::users::{PublicUser, UserStatus},
    policies::Policy,
};

#[get("/")]
pub async fn list_all_users(
    guard: JwtGuard,
    db: Db,
) -> Result<Success<Vec<PublicUser>>, Error<Null>> {
    let user = guard.get_user();

    let users = if user.is_admin() {
        // If the user is admin just return all users that exist
        database::users::get_all_public_users(&db).await?
    } else {
        database::users::get_all_public_users_from_workspaces(&db, user.id).await?
    };

    Ok(ApiResponse::success(
        format!("{} users found", users.len()),
        Some(users),
    ))
}

//Instead of get_paginated_users, maybe browse_users or list_users_paginated — to match REST semantics more intuitively.
#[get("/browse?<status>&<role>", format = "json", data = "<params>")]
pub async fn get_paginated_users(
    status: Option<i16>,
    role: Option<i16>,
    params: Json<PaginationRequest<UserField>>,
    guard: JwtGuard,
    db: Db,
) -> Result<Success<PaginatedRecords<PublicUser>>, Error<Null>> {
    let page =
        database::users::get_users_paginated(&db, guard.get_user(), status, role, params).await?;

    Ok(ApiResponse::success(
        format!(
            "{} of {} users shown",
            page.records_on_page(),
            page.total_records(),
        ),
        Some(page),
    ))
}

#[get("/<username>")]
pub async fn get_user_by_username(
    username: &str,
    guard: JwtGuard,
    db: Db,
) -> Result<Success<PublicUser>, Error<Null>> {
    // First get the user (the ID is needed to for the policy)
    let user = database::users::get_user_by_username(&db, username).await?;

    // Make sure the user is allowed to see the requested information
    Policy::users_get(&db, &guard.get_user(), user.id).await?;

    Ok(ApiResponse::success(
        format!("User '{username}' found"),
        Some(PublicUser::from(&user)),
    ))
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
    let user = database::users::get_user_by_id(&db, user_id).await?;

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
