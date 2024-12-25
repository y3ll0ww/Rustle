pub mod user;

pub fn user() -> Vec<rocket::Route> {
    routes![
        user::submit,
        user::create,
        user::get,
        user::delete
    ]
}