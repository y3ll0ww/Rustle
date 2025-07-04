use rocket::http::CookieJar;
use serde::de::DeserializeOwned;

use crate::api::{ApiResponse, Error};

pub mod permissions;

pub const TOKEN_COOKIE: &str = "auth_token";
pub const PROJECT_COOKIE: &str = "projects";
pub const WORKSPACE_COOKIE: &str = "workspaces";

/// Retrieves a deserialized copy of the data from the cookie.
pub fn get_cookie<T: DeserializeOwned>(
    cookie_key: &str,
    cookies: &CookieJar<'_>,
) -> Result<T, Error<String>> {
    match cookies.get_private(cookie_key) {
        Some(cookie) => {
            let cookie_value = cookie.value().to_string(); // Convert to String
            serde_json::from_str::<T>(&cookie_value).map_err(|e| {
                ApiResponse::<String>::internal_server_error(format!(
                    "Couldn't deserialize the cookie: {e}"
                ))
            })
        }
        None => Err(ApiResponse::bad_request(format!(
            "No '{cookie_key}' cookie found"
        ))),
    }
}
