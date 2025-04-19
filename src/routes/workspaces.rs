use diesel::prelude::*;
use rocket::{form::Form, http::CookieJar, State};
use rocket_sync_db_pools::diesel;
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{
        workspaces::{
            cache_key_workspace, CACHE_TTL_ONE_HOUR,
        },
        RedisMutex,
    },
    cookies::workspaces::{
        add_workspace_timestamp, get_workspace_timestamp, remove_workspace_update_cookie,
    },
    database::Db,
    forms::workspace::NewWorkspaceForm,
    models::{
        users::UserRole,
        workspaces::{MemberInfo, Workspace, WorkspaceWithMembers},
    },
    schema::{users, workspace_members, workspaces},
};

mod delete;
mod get;
mod post;
mod put;

// * /workspaces               -> GET
// * /workspaces/new           -> POST
// * /workspaces/<id>          -> GET
// * /workspaces/<id>/update   -> PUT
// * /workspaces/<id>/delete   -> DELETE
pub fn routes() -> Vec<rocket::Route> {
    routes![
        overview,
        post::create_new_workspace_by_form,
        get_workspace,
        put::update_workspace,
        delete_workspace,
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
