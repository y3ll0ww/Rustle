use super::*;

/// Returns an overview of teams of which the request user is a member.
///
/// ## Request
/// * Method: `GET`
/// * Guarded by JWT token
/// * Database access
///
/// ## Response
/// * **200 Created**: Returns a vector of [`Team`]s.
/// * **401 Unauthorized**: No [`TOKEN_COOKIE`](crate::cookies::TOKEN_COOKIE).
/// * **500 Server Error**: Any database operation fails.
pub async fn get_teams_by_user_id(
    guard: JwtGuard,
    db: Database,
) -> Result<Success<Vec<Team>>, Error<Null>> {
    let user_id = guard.get_user().id;

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
        .map_err(ApiResponse::from_error)?;

    // Return vector of teams
    Ok(ApiResponse::success(success_message, Some(teams)))
}

/// Returns information about a team, including its members.
///
/// ## Request
/// * Method: `GET`
/// * Guarded by JWT token
/// * Database access
/// * Cookies: [`USER_COOKIE`](crate::cookies::USER_COOKIE)
/// * Cache: [`team_cache_key`] with [`TEAM_CACHE_TTL`]
///
/// ## Response
/// * **200 Created**: Returns [`TeamWithMembers`].
///   - If there is a [`TeamUpdate`] in the cookie and the timestamp is the same as the timestamp
///     retrieved from the database, it returns from the **cache** (if it exists).
///   - If not; it retrieves from the database, adds the [`TeamUpdate`] to the cookies and the
///     [`TeamWithMembers`] to the cache (which it also returns in the response).
/// * **401 Unauthorized**: No [`TOKEN_COOKIE`](crate::cookies::TOKEN_COOKIE).
/// * **404 Not found**: No [`TeamUpdate`], [`TeamMember`]s or [`Team`] in the database.
/// * **500 Server Error**: Any database operation fails.
pub async fn get_team_by_id(
    id: Uuid,
    _guard: JwtGuard,
    db: Database,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<TeamWithMembers>, Error<Null>> {
    let redis = redis.lock().await;

    // Step 1: Check the team update stored in cookies
    let team_update_from_cookie = get_team_update_cookie(&id, cookies).unwrap_or_default();

    // Step 2: Get the team update from the database
    let team_id = id;
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
    let team_id = id;
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
                    team_members::team_role,
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
