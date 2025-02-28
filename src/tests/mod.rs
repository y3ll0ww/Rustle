use rocket::local::blocking::Client;

pub mod teams;
pub mod users;

pub fn test_client() -> Client {
    Client::tracked(crate::rocket()).expect("valid rocket instance")
}
