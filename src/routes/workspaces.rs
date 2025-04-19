use chrono::Utc;
use diesel::prelude::*;
use rocket::{form::Form, http::CookieJar, State};
use rocket_sync_db_pools::diesel;
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{
        workspaces::{
            set_workspace_cache, update_workspace_cache, workspace_cache_key, CACHE_TTL_ONE_HOUR,
        },
        RedisMutex,
    },
    cookies::workspaces::{
        add_workspace_update_cookie, get_workspace_update_cookie, remove_workspace_update_cookie,
    },
    database::Db,
    forms::workspace::{NewWorkspaceForm, UpdateWorkspaceForm},
    models::{
        users::UserRole,
        workspaces::{MemberInfo, Workspace, WorkspaceMember, WorkspaceRole, WorkspaceWithMembers},
    },
    schema::{users, workspace_members, workspaces},
};

mod delete;
mod get;
mod post;

// * /workspaces               -> GET
// * /workspaces/new           -> POST
// * /workspaces/<id>          -> GET
// * /workspaces/<id>/update   -> PUT
// * /workspaces/<id>/delete   -> DELETE
pub fn routes() -> Vec<rocket::Route> {
    routes![
        overview,
        new_workspace,
        get_workspace,
        update_workspace,
        delete_workspace
    ]
}

#[get("/")]
async fn overview(guard: JwtGuard, db: Db) -> Result<Success<Vec<Workspace>>, Error<Null>> {
    get::get_workspaces_by_user_id(guard, db).await
}

#[get("/<id>")]
async fn get_workspace(
    id: Uuid,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<WorkspaceWithMembers>, Error<Null>> {
    get::get_workspace_by_id(id, guard, db, cookies, redis).await
}

#[post("/new", data = "<form>")]
async fn new_workspace(
    form: Form<NewWorkspaceForm>,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    post::create_new_workspace_by_form(form, guard, db, cookies, redis).await
}

#[post("/<id>/update", data = "<form>")]
async fn update_workspace(
    id: Uuid,
    form: Form<UpdateWorkspaceForm>,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    post::update_workspace_by_form(id, form, guard, db, cookies, redis).await
}

#[delete("/<id>/delete")]
async fn delete_workspace(
    id: Uuid,
    guard: JwtGuard,
    db: Db,
    cookies: &CookieJar<'_>,
    redis: &State<RedisMutex>,
) -> Result<Success<Null>, Error<Null>> {
    delete::delete_workspace_by_id(id, guard, db, cookies, redis).await
}
