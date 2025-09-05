use routes::{USERS, WORKSPACES};

use crate::routes::PROJECTS;

#[macro_use]
extern crate rocket;

pub mod api;
pub mod auth;
pub mod cache;
pub mod cookies;
pub mod database;
pub mod email;
pub mod forms;
pub mod models;
pub mod policies;
pub mod routes;
pub mod schema;
#[cfg(test)]
mod tests;

pub const ENV_REDIS_URL: &str = "REDIS_URL";
pub const ENV_DATABASE_URL: &str = "DATABASE_URL";

#[launch]
fn rocket() -> _ {
    // Fetch DATABASE_URL from env
    let db_url = std::env::var(ENV_DATABASE_URL).expect(&format!("{ENV_DATABASE_URL} must be set"));

    // Merge it into Rocket's config at runtime
    let figment = rocket::Config::figment().merge(("databases.rustle_db.url", db_url));

    rocket::custom(figment)
        .attach(database::Db::fairing())
        .attach(cache::redis_fairing())
        .mount(PROJECTS, routes::projects::routes())
        .mount(USERS, routes::users::routes())
        .mount(WORKSPACES, routes::workspaces::routes())
}
