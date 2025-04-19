use rocket::{form::Form, State};

use crate::{
    api::{ApiResponse, Error, Null, Success},
    cache::{self, users::get_invite_token, RedisMutex},
    database::{users as database, Db},
    forms::password::Password,
};

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
    if database::set_user_password(&db, user_id, password_hash).await? == 0 {
        return Err(ApiResponse::bad_request(format!(
            "User '{user_id}' not affected"
        )));
    }

    // Remove the invitation token from the cache
    cache::users::remove_invite_token(redis, token)
        .await
        .map(|()| ApiResponse::success(format!("User '{user_id}' successfully activated"), None))
}
