use cache::redis_fairing;
use routes::{USERS, WORKSPACES};

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

#[launch]
fn rocket() -> _ {
    rocket::custom(rocket::Config::figment())
        .attach(database::Db::fairing())
        .attach(redis_fairing())
        .mount(WORKSPACES, routes::workspaces::routes())
        .mount(USERS, routes::users::routes())
}
