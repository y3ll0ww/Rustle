pub mod endpoint;
pub mod form;
pub mod model;

#[cfg(test)]
mod tests;

pub fn endpoints() -> Vec<rocket::Route> {
    routes![
        endpoint::submit,
        endpoint::create_user,
        endpoint::get_user,
        endpoint::delete_user
    ]
}
