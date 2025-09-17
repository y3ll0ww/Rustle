use std::collections::HashMap;

use rocket::{
    http::{Cookie, CookieJar, Status},
    response::status::Custom,
};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null},
    cookies::{get_cookie, PROJECT_COOKIE, WORKSPACE_COOKIE},
};

// ==============================================================
// PROJECT PERMISSIONS
// ==============================================================
//
pub fn insert_project_permission(
    id: Uuid,
    permission: i16,
    cookies: &CookieJar<'_>,
) -> Result<(), Error<Null>> {
    let cookie_key = PROJECT_COOKIE;
    insert_permission(id, permission, cookie_key, cookies)
}

pub fn get_project_permission(id: Uuid, cookies: &CookieJar<'_>) -> Result<i16, Error<Null>> {
    let cookie_key = PROJECT_COOKIE;
    get_permission(id, cookie_key, cookies)
}

// ==============================================================
// WORKSPACE PERMISSIONS
// ==============================================================
//
pub fn insert_workspace_permission(
    id: Uuid,
    permission: i16,
    cookies: &CookieJar<'_>,
) -> Result<(), Error<Null>> {
    let cookie_key = WORKSPACE_COOKIE;
    insert_permission(id, permission, cookie_key, cookies)
}

pub fn get_workspace_permission(id: Uuid, cookies: &CookieJar<'_>) -> Result<i16, Error<Null>> {
    let cookie_key = WORKSPACE_COOKIE;
    get_permission(id, cookie_key, cookies)
}

// ==============================================================
// GLOBAL PERMISSION FUNCTIONS
// ==============================================================
//
fn insert_permission(
    id: Uuid,
    permission: i16,
    cookie_key: &str,
    cookies: &CookieJar<'_>,
) -> Result<(), Error<Null>> {
    // Get the existing permissions
    let mut permissions = get_permissions(cookie_key, cookies)?;

    // Add the permission to the existing hashmap
    permissions.insert(id, permission);

    // Serialize the hashmap
    let cookie_value = serde_json::to_string(&permissions)
        .map_err(|e| ApiResponse::internal_server_error(e.to_string()))?;

    // Add or replace the (old) cookie
    cookies.add_private(Cookie::new(cookie_key.to_string(), cookie_value));

    Ok(())
}

fn get_permission(id: Uuid, cookie_key: &str, cookies: &CookieJar<'_>) -> Result<i16, Error<Null>> {
    // Get the existing permissions
    let permissions = get_permissions(cookie_key, cookies)?;

    // Return the permission if it exists
    match permissions.get(&id) {
        Some(permission) => Ok(*permission),
        None => Err(ApiResponse::unauthorized(format!(
            "No permission defined in {cookie_key}"
        ))),
    }
}

fn get_permissions(
    cookie_key: &str,
    cookies: &CookieJar<'_>,
) -> Result<HashMap<Uuid, i16>, Error<Null>> {
    // Deserialize the existing permissions
    let permissions = match get_cookie::<HashMap<Uuid, i16>>(cookie_key, cookies) {
        Ok(permissions) => permissions,
        // If it's a bad request, it means the cookie doesn't exist yet, which is fine
        Err(Custom(status, _)) if status.code == Status::BadRequest.code => HashMap::new(),
        Err(e) => return Err(e),
    };

    Ok(permissions)
}
