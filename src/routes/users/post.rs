use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cookies::TOKEN_COOKIE,
    database::{users as database, Db},
    forms::{login::LoginForm, password::Password},
    models::users::{User, UserStatus},
};
use rocket::{form::Form, http::CookieJar, serde::json::Json};
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
