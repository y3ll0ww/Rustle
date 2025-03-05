use diesel::prelude::*;

use rocket::{form::Form, http::CookieJar, State};
use rocket_sync_db_pools::diesel;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{
        teams::{team_cache_key, TEAM_CACHE_TTL},
        RedisMutex,
    },
    cookies::{
        teams::{add_team_update_cookie, get_team_update},
        users::get_user_info,
        TEAM_COOKIE,
    },
    db::Database,
    forms::teams::NewTeamForm,
    models::teams::{Team, TeamMember, TeamMemberInfo, TeamRole, TeamUpdate, TeamWithMembers},
    schema::{team_members, team_updates, teams, users},
};

#[post("/new", data = "<form>")]
pub async fn new(
    _guard: JwtGuard,
    db: Database,
    form: Form<NewTeamForm>, // Keep the lifetime here, it's needed for Rocket
    cookies: &CookieJar<'_>,
) -> Result<Success<Null>, Error<Null>> {
    let user_id = get_user_info(cookies).await.map(|user_info| user_info.id)?;

    // Set the success message
    let success_message = format!("Team created: '{}'", form.team_name);

    let team_description = (!form.description.is_empty()).then(|| form.description.clone());

    // Create a new Team
    let new_team = Team::new(user_id.clone(), form.team_name.clone(), team_description);

    // Add the new User to the database
    db.run(move |conn| {
        // Insert new team into teams table
        diesel::insert_into(teams::table)
            .values(&new_team)
            .execute(conn)?;

        // Create a new team member
        let owner_membership = TeamMember {
            team_id: new_team.id.clone(),
            user_id: user_id.clone(),
            team_privilege: TeamRole::Owner as i32,
        };

        // Insert owner into team_members table
        diesel::insert_into(team_members::table)
            .values(&owner_membership)
            .execute(conn)?;

        let team_update = TeamUpdate {
            team_id: new_team.id.clone(),
            last_updated: new_team.updated_at.to_string(),
        };

        // Insert team update into database
        diesel::insert_into(team_updates::table)
            .values(team_update)
            .execute(conn)
    })
    .await
    .map_err(ApiResponse::from_error)?;

    // Return success response
    Ok(ApiResponse::success(success_message, None))
}

#[get("/overview")]
pub async fn overview(
    _guard: JwtGuard,
    cookies: &CookieJar<'_>,
    db: Database,
) -> Result<Success<Vec<Team>>, Error<Null>> {
    let user_id = get_user_info(cookies).await.map(|user_info| user_info.id)?;

    // Set the success message
    let success_message = format!("Teams for user '{user_id}'");

    // Retrieve all teams with the user ID
    let teams = db
        .run(move |conn| {
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

#[get("/<id>")]
pub async fn get_team(
    _guard: JwtGuard,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
    db: Database,
    id: String,
) -> Result<Success<TeamWithMembers>, Error<Null>> {
    let redis = redis.lock().await;

    // Step 1: Check the team update stored in cookies
    let team_update_from_cookie = get_team_update(&id, cookies).unwrap_or_default();

    // Step 2: Get the team update from the database
    let team_id = id.clone();
    let team_update_from_db = db
        .run(move |conn| {
            team_updates::table
                .filter(team_updates::team_id.eq(&team_id))
                .first::<TeamUpdate>(conn)
                .map_err(ApiResponse::from_error)
        })
        .await?;

    // Step 3: Determine whether to return from cache or from database
    match team_update_from_cookie {
        // If there is a team update in the cookie and the timestamp is the same as the timestamp
        // retrieved from the database, we can return the team from the cache (if it exists).
        Some(team_update) if team_update.last_updated == team_update_from_db.last_updated => {
            // Retrieve the team information from the cache (ignore error in case cache not working)
            let team_from_cache = redis
                .get_from_cache::<TeamWithMembers>(&team_cache_key(&id))
                .await
                .unwrap_or(None);

            // If there is team information in the cache, return it
            if let Some(team_with_members) = team_from_cache.as_ref() {
                return Ok(ApiResponse::success(
                    format!("Team '{}' from cache", team_with_members.team.team_name),
                    team_from_cache,
                ));
            }
        }
        // In any other case (no cookie or different timestamps); update the cookie
        _ => add_team_update_cookie(team_update_from_db, cookies)?,
    }

    // Step 4: Get the team information including team members from the database
    let team_id = id.clone();
    let team_from_database = db
        .run(move |conn| {
            let team = teams::table
                .filter(teams::id.eq(&team_id))
                .first::<Team>(conn)
                .map_err(ApiResponse::from_error)?;

            let members = team_members::table
                .inner_join(users::table.on(users::id.eq(team_members::user_id)))
                .filter(team_members::team_id.eq(&team_id))
                .select((
                    users::id,
                    users::username,
                    users::display_name,
                    users::avatar_url,
                    team_members::team_privilege,
                ))
                .load::<TeamMemberInfo>(conn)
                .map_err(ApiResponse::from_error)?;

            Ok(TeamWithMembers { team, members })
        })
        .await?;

    // Add the team with members to the cache (ignore error in case cache not working)
    let _ = redis
        .set_to_cache(&team_cache_key(&id), &team_from_database, TEAM_CACHE_TTL)
        .await;

    Ok(ApiResponse::success(
        format!("Team '{}' from database", team_from_database.team.team_name),
        Some(team_from_database),
    ))
}

#[delete("/<id>/delete")]
pub async fn delete(
    _guard: JwtGuard,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
    db: Database,
    id: String,
) -> Result<Success<Null>, Error<Null>> {
    // Get user cookie
    let user_id = get_user_info(cookies).await.map(|user_info| user_info.id)?;

    // 1. Get the team from database
    let team_id = id.clone();
    let team = db
        .run(move |conn| {
            teams::table
                .filter(teams::id.eq(&team_id))
                .first::<Team>(conn)
                .map_err(ApiResponse::from_error)
        })
        .await?;

    // 2. Verify the owner
    if user_id != team.owner_id {
        return Err(ApiResponse::unauthorized(format!(
            "User '{user_id}' not the owner",
        )));
    }

    // 3. Remove the relevant cookie
    cookies.remove_private(TEAM_COOKIE);

    // 4. Remove the team information in the cache; if this fails ignore
    let _ = redis
        .lock()
        .await
        .remove_from_cache(&team_cache_key(&id))
        .await;

    // 5. Remove the team from the database
    //    - Delete on cascade for team_members table
    //    - Delete on cascade for team_updates table
    let team_id = id.clone();
    let deleted_rows = db
        .run(move |conn| diesel::delete(teams::table.filter(teams::id.eq(&team_id))).execute(conn))
        .await
        .map_err(ApiResponse::from_error)?;

    if deleted_rows == 0 {
        return Err(ApiResponse::internal_server_error(
            "Nothing deleted".to_string(),
        ));
    }

    // Return success
    Ok(ApiResponse::success(
        format!("Team {} deleted", team.id),
        None,
    ))
}
