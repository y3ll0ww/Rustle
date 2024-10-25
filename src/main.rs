#[macro_use]
extern crate rocket;

use db::{DbConn, models::users};

mod db;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![users::create_user])
}