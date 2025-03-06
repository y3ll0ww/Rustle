use diesel::{
    prelude::*,
    result::{DatabaseErrorKind, Error as DieselError},
};

use rocket::{form::Form, http::CookieJar, serde::json::Json};
use rocket_sync_db_pools::diesel;
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cookies::{users::generate_and_add_cookies, TOKEN_COOKIE, USER_COOKIE},
    db::Database,
    forms::users::{LoginForm, NewUserForm, Password},
    models::users::User,
    schema::users,
};

mod delete;
mod get;
mod post;

// * /user               -> GET
// * /user/<id>          -> GET
// * /user/<id>/update   -> PUT
// * /user/<id>/delete   -> DELETE
// * /user/login         -> POST
// * /user/logout        -> POST
// * /user/register      -> POST
pub fn routes() -> Vec<rocket::Route> {
    routes![
        all_users,
        get_user,
        delete_user,
        login,
        logout,
        register,
        inject_new_user,
    ]
}

#[get("/")]
async fn all_users(db: Database) -> Result<Success<Vec<User>>, Error<Null>> {
    get::list_all_users(db).await
}

#[get("/<username>")]
async fn get_user(username: String, db: Database) -> Result<Success<User>, Error<Null>> {
    get::get_user_by_username(username, db).await
}

#[delete("/<id>/delete")]
async fn delete_user(
    id: String,
    guard: JwtGuard,
    db: Database,
    cookies: &CookieJar<'_>,
) -> Result<Success<Null>, Error<Null>> {
    delete::delete_user_by_id(id, guard, db, cookies).await
}

#[post("/login", data = "<credentials>")]
async fn login(
    credentials: Form<LoginForm<'_>>,
    db: Database,
    cookies: &CookieJar<'_>,
) -> Result<Success<Null>, Error<Null>> {
    post::login_by_form(credentials, db, cookies).await
}

#[post("/logout")]
fn logout(guard: JwtGuard, cookies: &CookieJar<'_>) -> Success<String> {
    post::logout(guard, cookies)
}

#[post("/register", data = "<form>")]
async fn register(
    form: Form<NewUserForm<'_>>,
    db: Database,
    cookies: &CookieJar<'_>,
) -> Result<Success<Null>, Error<Null>> {
    post::create_new_user_by_form(form, db, cookies).await
}

#[post("/create", format = "json", data = "<user>")]
async fn inject_new_user(user: Json<User>, db: Database) -> String {
    post::inject_user(user, db).await
}

async fn get_user_from_db(db: Database, username: String) -> Result<Success<User>, Error<Null>> {
    db.run(move |conn| {
        users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
    })
    .await
    .map(|user| ApiResponse::success(format!("User '{}' found", user.username), Some(user)))
    .map_err(|e| ApiResponse::not_found(e.to_string()))
}
