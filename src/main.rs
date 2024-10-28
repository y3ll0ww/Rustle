#[macro_use]
extern crate rocket;

use db::{api::users, Db};

mod db;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .mount("/users/", users::endpoints())
}
