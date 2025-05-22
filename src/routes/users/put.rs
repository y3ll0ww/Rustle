use rocket::{form::Form, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, users::get_invite_token, RedisMutex},
    database::{self, Db},
    forms::password::Password,
    models::users::{PublicUser, UserRole, UserStatus, UserUpdate},
    policies::Policy,
};

#[put("/update/<id>", format = "json", data = "<update>")]
pub async fn update_user(
    id: Uuid,
    update: Json<UserUpdate>,
    guard: JwtGuard,
    db: Db,
) -> Result<Success<PublicUser>, Error<Null>> {
    // Check if the user is authorized to perform this action
    Policy::users_update_info(&guard.get_user(), id)?;

    // Update the workspace information in the database
    let updated_user =
        database::users::update_user_information(&db, id, update.clone().into_inner()).await?;

    // Return a success response
    Ok(ApiResponse::success(
        "User updated successfully".to_string(),
        Some(updated_user),
    ))
}

#[put("/invite/set/<token>", data = "<form>")]
pub async fn set_password_after_invite(
    token: &str,
    form: Form<Password<'_>>,
    db: Db,
    redis: &State<RedisMutex>,
) -> Result<Success<Vec<String>>, Error<Null>> {
    // Get the user from the redis cache
    let user_id = get_invite_token(redis, token).await?;

    // Verify that the password input match
    if !form.inputs_match() {
        return Err(ApiResponse::bad_request(
            "Password inputs do not match".to_string(),
        ));
    };

    // Hash the provided password
    let password_hash = form
        .hash_password()
        .map_err(|e| ApiResponse::internal_server_error(format!("Couldn't hash password: {e}")))?;

    // Update the user and increment the UserStatus
    if database::users::set_user_password(&db, user_id, password_hash).await? == 0 {
        return Err(ApiResponse::bad_request(format!(
            "User '{user_id}' not affected"
        )));
    }

    // Remove the invitation token from the cache
    cache::users::remove_invite_token(redis, token)
        .await
        .map(|()| ApiResponse::success(format!("User '{user_id}' successfully activated"), None))
}

#[put("/update/<id>/<role>")]
pub async fn update_role(
    id: Uuid,
    role: i16,
    guard: JwtGuard,
    db: Db,
) -> Result<Success<PublicUser>, Error<Null>> {
    let user = guard.get_user();

    // Check if the user is authorized to perform this action
    Policy::users_set_role(&user, role)?;

    // Verify the validity of the user role
    let user_role = UserRole::try_from(role).map_err(ApiResponse::bad_request)?;

    // Update the user role
    let updated_user = database::users::update_user_role(&db, id, role).await?;

    Ok(ApiResponse::success(
        format!("User role: {user_role:?}"),
        Some(updated_user),
    ))
}

#[put("/update/<id>/suspend")]
pub async fn suspend_user(
    id: Uuid,
    guard: JwtGuard,
    db: Db,
) -> Result<Success<PublicUser>, Error<Null>> {
    user_status_update(&db, id, &guard.get_user(), UserStatus::Suspended).await
}

#[put("/update/<id>/remove")]
pub async fn remove_user(
    id: Uuid,
    guard: JwtGuard,
    db: Db,
) -> Result<Success<PublicUser>, Error<Null>> {
    user_status_update(&db, id, &guard.get_user(), UserStatus::Removed).await
}

async fn user_status_update(
    db: &Db,
    id: Uuid,
    user: &PublicUser,
    status: UserStatus,
) -> Result<Success<PublicUser>, Error<Null>> {
    // User must be at least a manager
    Policy::users_set_status(user)?;

    // Create message before it goes out of scope
    let message = format!("User status: {status:?}");

    // Update the user status
    let updated_user = database::users::update_user_status(db, id, i16::from(status)).await?;

    Ok(ApiResponse::success(message, Some(updated_user)))
}
