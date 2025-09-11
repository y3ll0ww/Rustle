use rocket::{fairing::AdHoc, Config};
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::{USERS, WORKSPACES};

use crate::{
    database::{users::setup_admin, Db},
    routes::PROJECTS,
};

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
pub const ENV_POSTGRES_USER: &str = "POSTGRES_USER";
pub const ENV_POSTGRES_PASSWORD: &str = "POSTGRES_PASSWORD";

pub fn env(key: &str) -> String {
    std::env::var(key).expect(&format!("Environment variable '{key}' missing"))
}

#[launch]
fn rocket() -> _ {
    // Configure CORS options
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .expect("error creating CORS fairing");

    // Fetch DATABASE_URL from env and merge it into Rocket's config at runtime
    rocket::custom(Config::figment().merge(("databases.rustle_db.url", env(ENV_DATABASE_URL))))
        .attach(cors)
        .attach(database::Db::fairing())
        .attach(cache::redis_fairing())
        .attach(create_admin())
        .mount(PROJECTS, routes::projects::routes())
        .mount(USERS, routes::users::routes())
        .mount(WORKSPACES, routes::workspaces::routes())
}

fn create_admin() -> AdHoc {
    AdHoc::on_ignite("Setup Admin user", |rocket| async {
        // Get database connection from state
        let db = Db::get_one(&rocket).await.expect("No database connection");

        // Create the initial admin user
        if let Err(e) = setup_admin(&db).await {
            eprintln!("{e}");
        };

        rocket
    })
}
