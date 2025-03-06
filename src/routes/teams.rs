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
    models::{
        teams::{Team, TeamMember, TeamMemberInfo, TeamRole, TeamUpdate, TeamWithMembers},
        users::UserRole,
    },
    schema::{team_members, team_updates, teams, users},
};

pub mod delete;
pub mod get;
pub mod post;

// * /teams               -> GET
// * /teams/new           -> POST
// * /teams/<id>          -> GET
// * /teams/<id>/update   -> PUT
// * /teams/<id>/delete   -> DELETE
pub fn routes() -> Vec<rocket::Route> {
    routes![overview, new, get_team, delete_team]
}

#[get("/")]
pub async fn overview(
    guard: JwtGuard,
    db: Database,
    cookies: &CookieJar<'_>,
) -> Result<Success<Vec<Team>>, Error<Null>> {
    get::get_teams_by_user_id(guard, db, cookies).await
}

#[get("/<id>")]
pub async fn get_team(
    id: String,
    guard: JwtGuard,
    db: Database,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<TeamWithMembers>, Error<Null>> {
    get::get_team_by_id(id, guard, db, cookies, redis).await
}

#[post("/new", data = "<form>")]
pub async fn new(
    form: Form<NewTeamForm>,
    guard: JwtGuard,
    db: Database,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    post::create_new_team_by_form(form, guard, db, cookies, redis).await
}

#[delete("/<id>/delete")]
pub async fn delete_team(
    id: String,
    guard: JwtGuard,
    db: Database,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    delete::delete_team_by_id(id, guard, db, cookies, redis).await
}
