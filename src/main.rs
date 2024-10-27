#[macro_use]
extern crate rocket;

use db::{Db, models::users};

mod db;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .mount("/users/", routes![users::create_user, users::get_user, users::delete_user])
}