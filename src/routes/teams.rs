use diesel::prelude::*;

use rocket::http::CookieJar;
use rocket_sync_db_pools::diesel;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::{JwtGuard, UserInfo, USER_COOKIE},
    db::Database,
    models::teams::Team,
    schema::{team_members, teams},
};

#[get("/overview")]
pub async fn overview(_guard: JwtGuard, cookies: &CookieJar<'_>, db: Database) -> Result<Success<Vec<Team>>, Error<Null>> {
    // Get user ID from cookie
    let user_id = match cookies.get_private(USER_COOKIE) {
        Some(cookie) => serde_json::from_str::<UserInfo>(cookie.value()),
        None => return Err(ApiResponse::bad_request("No user cookie found".to_string())),
    }
    .map(|user_info| user_info.id)
    .map_err(|e| ApiResponse::internal_server_error(format!("Couldn't deserialize the cookie: {e}")))?;

    // Set the success message
    let success_message = format!("Teams for user '{user_id}'");

    // Retrieve all teams with the user ID
    let teams = db.run(move |conn| {
        teams::table
            .inner_join(team_members::table.on(team_members::team_id.eq(teams::id)))
            .filter(team_members::user_id.eq(user_id))
            .select(teams::all_columns)
            .load::<Team>(conn)
    })
    .await
    .map_err(|e| ApiResponse::internal_server_error(e.to_string()))?;

    // Return vector of teams
    Ok(ApiResponse::success(success_message, Some(teams)))
}
