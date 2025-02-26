use redis::redis_fairing;

#[macro_use]
extern crate rocket;

pub mod api;
pub mod auth;
pub mod db;
pub mod forms;
pub mod models;
pub mod redis;
pub mod routes;
pub mod schema;
#[cfg(test)]
mod tests;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::Database::fairing())
        .attach(redis_fairing())
        .mount("/user/", routes::user())
}
