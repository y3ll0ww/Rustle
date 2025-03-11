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
    cookies::teams::{add_team_update_cookie, get_team_update_cookie, remove_team_update_cookie},
    db::Database,
    forms::teams::{NewTeamForm, UpdateTeamForm},
    models::{
        teams::{Team, TeamMember, TeamMemberInfo, TeamRole, TeamUpdate, TeamWithMembers},
        users::UserRole,
    },
    schema::{team_members, team_updates, teams, users},
};

mod delete;
mod get;
mod post;

// * /teams               -> GET
// * /teams/new           -> POST
// * /teams/<id>          -> GET
// * /teams/<id>/update   -> PUT
// * /teams/<id>/delete   -> DELETE
pub fn routes() -> Vec<rocket::Route> {
    routes![overview, new_team, get_team, update_team, delete_team]
}

#[get("/")]
async fn overview(guard: JwtGuard, db: Database) -> Result<Success<Vec<Team>>, Error<Null>> {
    get::get_teams_by_user_id(guard, db).await
}

#[get("/<id>")]
async fn get_team(
    id: String,
    guard: JwtGuard,
    db: Database,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<TeamWithMembers>, Error<Null>> {
    get::get_team_by_id(id, guard, db, cookies, redis).await
}

#[post("/new", data = "<form>")]
async fn new_team(
    form: Form<NewTeamForm>,
    guard: JwtGuard,
    db: Database,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    post::create_new_team_by_form(form, guard, db, cookies, redis).await
}

#[post("/<id>/update", data = "<form>")]
async fn update_team(
    id: String,
    form: Form<UpdateTeamForm>,
    guard: JwtGuard,
    db: Database,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    post::update_team_by_form(id, form, guard, db, cookies, redis).await
}

#[delete("/<id>/delete")]
async fn delete_team(
    id: String,
    guard: JwtGuard,
    db: Database,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    delete::delete_team_by_id(id, guard, db, cookies, redis).await
}
