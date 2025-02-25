#[macro_use]
extern crate rocket;

pub mod api;
pub mod auth;
pub mod db;
pub mod forms;
pub mod models;
pub mod routes;
pub mod schema;
#[cfg(test)]
mod tests;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::Database::fairing())
        .mount("/user/", routes::user())
}
