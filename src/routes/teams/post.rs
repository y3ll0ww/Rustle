//use chrono::Utc;
//
//use crate::{
//    cache::teams::{set_team_cache, update_team_cache},
//    forms::teams::UpdateTeamForm,
//};
//
use super::*;

/// Creates a new team.
///
/// ## Permissions
/// Requires as least [`UserRole::Manager`] to successfully complete.
///
/// ## Request
/// * Method: `POST`
/// * Guarded by JWT token
/// * Data: [`NewTeamForm`]
/// * Database access
/// * Cookies: [`TEAM_COOKIE`]
/// * Cache: [`team_cache_key`] with [`TEAM_CACHE_TTL`]
///
/// ## Response
/// * **201 Created**: Nothing returned.
///   - [`Team`] added to [`teams::table`] with request user as `owner_id`.
///   - Request user (from cookie) added as [`TeamMember`] with [`TeamRole::Owner`] to
///     [`team_members::table`].
///   - [`TeamUpdate`] added to [`team_updates::table`] with `updated_at`.
///   - [`TeamUpdate`] is added as a cookie.
///   - [`TeamWithMembers`] added to the **Redis** cache (ignore if fail).
/// * **401 Unauthorized**:
///   - No [`TOKEN_COOKIE`](crate::cookies::TOKEN_COOKIE).
///   - Request user is not [`UserRole::Manager`] or higher.
/// * **500 Server Error**: Any database operation fails.
pub async fn create_new_team_by_form(
    form: Form<NewTeamForm>,
    guard: JwtGuard,
    db: Database,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    // Get user information from cookies
    let user = guard.get_user();

    // Step 1: Validate user permissions
    if (user.role.clone() as i32) < UserRole::Manager as i32 {
        return Err(ApiResponse::unauthorized(
            "User not allowed to create teams".to_string(),
        ));
    }

    // Create a new Team
    let new_team = Team::new(
        user.id.clone(),
        form.team_name.clone(),
        form.description.clone(),
    );

    // Create a new team member
    let owner_membership = TeamMember {
        team_id: new_team.id.clone(),
        user_id: user.id.clone(),
        team_role: TeamRole::Owner as i16,
    };

    // Create a new team update
    let team_update = TeamUpdate {
        team_id: new_team.id.clone(),
        last_updated: new_team.updated_at,
    };

    // Create some variables from which types will go out of scope
    let success_message = format!("Team created: '{}'", form.team_name);
    let team_id = new_team.id.clone();
    let user = user.clone();
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
                    team_role: owner_membership.team_role,
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
    set_team_cache(redis, &team_id, &cache_team_with_members).await;

    // Return success response
    Ok(ApiResponse::success(success_message, None))
}

pub async fn update_team_by_form(
    id: Uuid,
    form: Form<UpdateTeamForm>,
    guard: JwtGuard,
    db: Database,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    // User must be a manager
    let minimal_user_role = UserRole::Manager;
    let minimal_team_role = TeamRole::Contributor;

    // Get user information from cookies
    let user = guard.get_user();

    // Copy some values to prevent borrowing issues
    let team_id = id.clone();
    let form_clone = form.clone();

    // Perform database actions
    let team_update = db
        .run(move |conn| {
            // Step 1: Get the user information from the team members table
            let team_member = team_members::table
                .filter(team_members::team_id.eq(&team_id))
                .filter(team_members::user_id.eq(&user.id))
                .first::<TeamMember>(conn)
                .map_err(ApiResponse::from_error)?;

            // Step 2: Validate if the user has permission to update the team
            if team_member.team_role < minimal_team_role as i16
                && (user.role as i16) < minimal_user_role as i16
            {
                return Err(ApiResponse::unauthorized(
                    "No permission to update team information".to_string(),
                ));
            }

            // Set the current timestamp
            let timestamp = Utc::now().naive_utc();

            // Step 3: Update the team table with information from the form
            if diesel::update(teams::table.filter(teams::id.eq(&team_id)))
                .set((&*form, teams::updated_at.eq(timestamp)))
                .execute(conn)
                .map_err(ApiResponse::from_error)?
                == 0
            {
                return Err(ApiResponse::not_found("No teams to update".to_string()));
            };

            // Step 4: Update the team updates table to reflect the change
            if diesel::update(team_updates::table.filter(team_updates::team_id.eq(&team_id)))
                .set(team_updates::last_updated.eq(timestamp))
                .execute(conn)
                .map_err(ApiResponse::from_error)?
                == 0
            {
                return Err(ApiResponse::not_found(
                    "No team updates to update".to_string(),
                ));
            };

            Ok(TeamUpdate {
                team_id,
                last_updated: timestamp,
            })
        })
        .await?;

    // Step 5: Update the team in the cache
    if update_team_cache(redis, &id, Some(form_clone), None).await {
        // Step 6: If cache has been updated, update the cookie too
        add_team_update_cookie(team_update, cookies)?;
    }

    // Return a success response
    Ok(ApiResponse::success(
        "Team updated successfully".to_string(),
        None,
    ))
}

//pub id: String,
//pub owner_id: String,
//pub team_name: String,
//pub team_description: Option<String>,
//pub image_url: Option<String>,
//pub created_at: NaiveDateTime,
//pub updated_at: NaiveDateTime,

// Change owner
// Add member
