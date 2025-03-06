use super::*;

/// Creates a new team.
///
/// ## Permissions
/// Requires as least [`UserRole::Manager`] to successfully complete.
///
/// ## Request
/// * Method: `POST`
/// * Data: [`NewTeamForm`]
/// * Database access
/// * Cookies: [`USER_COOKIE`]
///
/// ## Response
/// * **201 Created**:
///   - [`Team`] added to [`teams::table`] with request user as `owner_id`.
///   - Request user (from cookie) added as [`TeamMember`] with [`TeamRole::Owner`] to
///     [`team_members::table`].
///   - [`TeamUpdate`] added to [`team_updates::table`] with `updated_at`.
///   - [`TeamUpdate`] is added as a cookie.
///   - [`TeamWithMembers`] added to the **Redis** cache (ignore if fail).
/// * **401 Unauthorized**: Request user is not [`UserRole::Manager`] or higher.
/// * **500 Server Error**: Any database operation fails.
#[post("/new", data = "<form>")]
pub async fn create_new_team_by_form(
    form: Form<NewTeamForm>,
    _guard: JwtGuard,
    db: Database,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    // Get user information from cookies
    let user_info = get_user_info(cookies).await?;

    // Step 1: Validate user permissions
    if (user_info.role.clone() as i32) < UserRole::Manager as i32 {
        return Err(ApiResponse::unauthorized(
            "User not allowed to create teams".to_string(),
        ));
    }

    // Create a new Team
    let new_team = Team::new(
        user_info.id.clone(),
        form.team_name.clone(),
        form.description.clone(),
    );

    // Create a new team member
    let owner_membership = TeamMember {
        team_id: new_team.id.clone(),
        user_id: user_info.id.clone(),
        team_privilege: TeamRole::Owner as i32,
    };

    // Create a new team update
    let team_update = TeamUpdate {
        team_id: new_team.id.clone(),
        last_updated: new_team.updated_at.to_string(),
    };

    // Create some variables from which types will go out of scope
    let success_message = format!("Team created: '{}'", form.team_name);
    let team_id = new_team.id.clone();
    let user = user_info.clone();
    let team_update_clone = team_update.clone();

    // Step 2: Add new team to database
    let cache_team_with_members = db
        .run(move |conn| {
            // Get additional user information
            let (display_name, avatar_url) = users::table
                .filter(users::id.eq(&user.id))
                .select((users::display_name, users::avatar_url))
                .first::<(Option<String>, Option<String>)>(conn)?;

            // Create a new team with members to place in the cache
            let team_with_members = TeamWithMembers {
                team: new_team.clone(),
                members: vec![TeamMemberInfo {
                    user_id: user.id,
                    username: user.username,
                    display_name,
                    avatar_url,
                    team_privilege: owner_membership.team_privilege,
                }],
            };

            // Insert new team into teams table
            diesel::insert_into(teams::table)
                .values(&new_team)
                .execute(conn)?;

            // Insert owner into team_members table
            diesel::insert_into(team_members::table)
                .values(&owner_membership)
                .execute(conn)?;

            // Insert team update into team_updates table
            diesel::insert_into(team_updates::table)
                .values(&team_update_clone)
                .execute(conn)?;

            // Return the team with members for caching
            Ok(team_with_members)
        })
        .await
        .map_err(ApiResponse::from_error)?;

    // Step 3: Add the team update to the cookies
    add_team_update_cookie(team_update, cookies)?;

    // Step 4: Add the team information to the cache
    let _ = redis
        .lock()
        .await
        .set_to_cache(
            &team_cache_key(&team_id),
            &cache_team_with_members,
            TEAM_CACHE_TTL,
        )
        .await;

    // Return success response
    Ok(ApiResponse::success(success_message, None))
}
