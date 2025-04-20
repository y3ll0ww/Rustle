use std::collections::HashMap;

use rocket::{
    http::{CookieJar, Status},
    response::status::Custom,
};
use uuid::Uuid;

use crate::api::{ApiResponse, Error, Null};

use super::{get_cookie, WORKSPACE_COOKIE};

pub fn insert_workspace_permission(
    cookies: &CookieJar<'_>,
    workspace: Uuid,
    permission: i16,
) -> Result<(), Error<Null>> {
    // Get the existing workspace permissions
    let mut workspace_permissions = get_workspace_permissions(cookies)?;

    // Add the workspace permission to the existing hashmap
    workspace_permissions.insert(workspace, permission);

    // Serialize the hashmap
    let cookie = serde_json::to_string(&workspace_permissions)
        .map_err(|e| ApiResponse::internal_server_error(e.to_string()))?;

    // Add or replace the (old) cookie
    cookies.add_private(cookie);

    Ok(())
}

pub fn get_workspace_permission(
    cookies: &CookieJar<'_>,
    workspace: Uuid,
) -> Result<i16, Error<Null>> {
    // Get the existing workspace permissions
    let workspace_permissions = get_workspace_permissions(cookies)?;

    // Return the permission if it exists
    match workspace_permissions.get(&workspace) {
        Some(permission) => Ok(*permission),
        None => Err(ApiResponse::unauthorized(
            "No permission defined for this workspace".to_string(),
        )),
    }
}

fn get_workspace_permissions(cookies: &CookieJar<'_>) -> Result<HashMap<Uuid, i16>, Error<Null>> {
    // Deserialize the existing team updates
    let workspace_permissions = match get_cookie::<HashMap<Uuid, i16>>(WORKSPACE_COOKIE, cookies) {
        Ok(permissions) => permissions,
        // If it's a bad request, it means the cookie doesn't exist yet, which is fine
        Err(Custom(status, _)) if status.code == Status::BadRequest.code => HashMap::new(),
        Err(e) => return Err(e),
    };

    Ok(workspace_permissions)
}
