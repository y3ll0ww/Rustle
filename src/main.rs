use cache::redis_fairing;
use routes::{TEAMS, USERS};

#[macro_use]
extern crate rocket;

pub mod api;
pub mod auth;
pub mod cache;
pub mod cookies;
pub mod db;
pub mod forms;
pub mod models;
pub mod routes;
pub mod schema;
#[cfg(test)]
mod tests;

#[launch]
fn rocket() -> _ {
    rocket::custom(rocket::Config::figment())
        .attach(db::Database::fairing())
        .attach(redis_fairing())
        .mount(TEAMS, routes::teams::routes())
        .mount(USERS, routes::users::routes())
}
