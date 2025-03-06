use rocket::{
    http::{Cookie, CookieJar, Status},
    response::status::Custom,
};

use crate::{
    api::{ApiResponse, Error},
    models::teams::TeamUpdate,
};

use super::{get_cookie, TEAM_COOKIE};

pub fn add_team_update_cookie(
    team_update: TeamUpdate,
    cookies: &CookieJar<'_>,
) -> Result<(), Error<String>> {
    // Deserialize the existing team updates
    let mut team_updates = match get_cookie::<Vec<TeamUpdate>>(TEAM_COOKIE, cookies) {
        Ok(team_updates) => team_updates,
        // If it's a bad request, it means the cookie doesn't exist yet, which is fine
        Err(Custom(status, _)) if status.code == Status::BadRequest.code => Vec::new(),
        Err(err) => return Err(err),
    };

    // Add the new team update, or update an existing one
    match team_updates
        .iter_mut()
        .find(|update| update.team_id == team_update.team_id)
    {
        Some(existing_update) => *existing_update = team_update,
        None => team_updates.push(team_update),
    }

    update_cookie(team_updates, cookies)
}

pub fn get_team_update_cookie(
    team_id: &str,
    cookies: &CookieJar<'_>,
) -> Result<Option<TeamUpdate>, Error<String>> {
    // Deserialize the existing team updates
    let team_updates = match get_cookie::<Vec<TeamUpdate>>(TEAM_COOKIE, cookies) {
        Ok(updates) => updates,
        // If it's a bad request, it means the cookie doesn't exist yet, which is fine
        Err(Custom(status, _)) if status.code == Status::BadRequest.code => return Ok(None),
        Err(err) => return Err(err),
    };

    let team_update = team_updates
        .into_iter()
        .find(|update| update.team_id == team_id);

    Ok(team_update)
}

pub fn remove_team_update_cookie(
    team_id: &str,
    cookies: &CookieJar<'_>,
) -> Result<(), Error<String>> {
    // Deserialize the existing team updates
    let mut team_updates = all_team_updates(cookies)?;

    // Retain all team updates that don't have the provided team ID
    team_updates.retain(|team_update| team_update.team_id != team_id);

    update_cookie(team_updates, cookies)
}

fn all_team_updates(cookies: &CookieJar<'_>) -> Result<Vec<TeamUpdate>, Error<String>> {
    match get_cookie::<Vec<TeamUpdate>>(TEAM_COOKIE, cookies) {
        Ok(team_updates) => Ok(team_updates),
        // If it's a bad request, it means the cookie doesn't exist yet, which is fine
        Err(Custom(status, _)) if status.code == Status::BadRequest.code => Ok(Vec::new()),
        Err(err) => Err(err),
    }
}

fn update_cookie(
    team_updates: Vec<TeamUpdate>,
    cookies: &CookieJar<'_>,
) -> Result<(), Error<String>> {
    // Serialize the vector with team updates
    let serialized_team_updates = serde_json::to_string(&team_updates)
        .map_err(|e| ApiResponse::internal_server_error(e.to_string()))?;

    // Create the cookie
    let team_update_cookie = Cookie::new(TEAM_COOKIE, serialized_team_updates);

    // Add or overwrite the existing cookie
    cookies.add_private(team_update_cookie);

    Ok(())
}
