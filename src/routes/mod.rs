pub mod users;

pub fn user() -> Vec<rocket::Route> {
    routes![
        users::submit,
        users::create,
        users::get,
        users::delete
    ]
}
