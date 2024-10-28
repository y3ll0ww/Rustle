#[macro_use]
extern crate rocket;

pub mod api;
pub mod db;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::Db::fairing())
        .mount("/users/", api::users::endpoints())
}
