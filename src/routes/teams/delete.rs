use super::*;

pub async fn delete_team_by_id(
    id: String,
    _guard: JwtGuard,
    db: Database,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
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
