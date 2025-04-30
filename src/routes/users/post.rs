use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    cookies::TOKEN_COOKIE,
    database::{users as database, Db},
    email::MailClient,
    forms::{login::LoginForm, password::Password},
    models::users::{PublicUser, User, UserStatus},
};
use rocket::{form::Form, http::CookieJar, serde::json::Json, State};
use uuid::Uuid;

#[post("/login", data = "<credentials>")]
pub async fn login_by_form(
    credentials: Form<LoginForm<'_>>,
    db: Db,
    cookies: &CookieJar<'_>,
) -> Result<Success<Null>, Error<Null>> {
    // Get the user from the database
    let user = database::get_user_by_username(&db, credentials.username).await?;

    // Return not found if the user is not active
    if user.status != i16::from(UserStatus::Active) {
        return Err(ApiResponse::not_found(format!(
            "User '{}' not found",
            user.username
        )));
    }

    // Validate if the given password is correct
    if !Password::verify_password(credentials.password, &user.password).map_err(|e| {
        ApiResponse::internal_server_error(format!("Password verification failed: {}", e))
    })? {
        return Err(ApiResponse::bad_request("Invalid password".to_string()));
    };

    // Add the user to the JWT guard
    JwtGuard::secure(&user, cookies)
        .await
        .map_err(ApiResponse::internal_server_error)?;

    Ok(ApiResponse::success("Login successful".to_string(), None))
}

#[post("/logout")]
pub fn logout(_guard: JwtGuard, cookies: &CookieJar<'_>) -> Success<String> {
    cookies.remove_private(TOKEN_COOKIE);

    ApiResponse::success(
        "Logout successful - token and user info removed".to_string(),
        None,
    )
}

/// TODO!:
/// - Adding space/project functionality
/// - Inviting only when a certain role in space
#[post("/invite/re/<space>/<id>")]
pub async fn reinvite_user_by_id(
    space: &str,
    id: Uuid,
    guard: JwtGuard,
    db: Db,
    redis: &State<RedisMutex>,
) -> Result<Success<String>, Error<Null>> {
    // Get the user from the database
    let user = database::get_user_by_id(&db, id).await?;

    // Extract the user status
    let user_status =
        UserStatus::try_from(user.status).map_err(|e| ApiResponse::conflict(e, String::new()))?;

    // Make sure the user status is still on invited
    if !matches!(user_status, UserStatus::Invited) {
        return Err(ApiResponse::bad_request(format!(
            "User {} has status {user_status:?}",
            user.username,
        )));
    };

    // Create a random token with a length of 64 characters
    let token = cache::create_random_token(64);

    // Add the token to the redis cache; containing the user ID
    cache::users::add_invite_token(redis, &token, user.id).await?;

    // Get the required information for the invitation email
    let inviter = guard.get_user();
    let recipient = PublicUser::from(&user);
    let workspace_name = space.replace('_', " ");

    // Send the email
    MailClient::no_reply()
        .send_invitation(&inviter, &recipient, &workspace_name, &token)
        .map_err(ApiResponse::internal_server_error)?;

    Ok(ApiResponse::success(
        format!("{} invited", user.username),
        Some(token),
    ))
}

#[post("/create", format = "json", data = "<user>")]
pub async fn inject_user(user: Json<User>, db: Db) -> String {
    let mut new_user = user.into_inner(); // Extract user data from Json
    let username = new_user.username.clone();
    new_user.id = Uuid::new_v4(); // Generate a new UUID

    // Use Diesel to insert the new user
    match database::inject_user(&db, new_user).await {
        Ok(_) => format!("User {username} created"),
        Err(e) => format!("Error creating user: {e}"), // Print error details
    }
}
