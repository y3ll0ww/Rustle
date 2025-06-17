use rocket::local::{asynchronous::Client as AsyncClient, blocking::Client};

pub mod projects;
pub mod users;
pub mod workspaces;

pub fn test_client() -> Client {
    Client::tracked(crate::rocket()).expect("valid rocket instance")
}

pub async fn async_test_client() -> AsyncClient {
    AsyncClient::tracked(crate::rocket())
        .await
        .expect("valid rocket instance")
}
