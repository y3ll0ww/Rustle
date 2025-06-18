use rocket::{
    http::Status,
    local::{
        asynchronous::Client as AsyncClient,
        blocking::{Client, LocalResponse},
    },
};
use serde_json::Value;

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

pub fn response_ok(response: LocalResponse<'_>) {
    // Extract the status
    let status = response.status().clone();

    // Format and print the response
    let body = response.into_string().unwrap_or("{}".to_string());
    let value = serde_json::from_str::<Value>(&body).unwrap();
    println!("{}", serde_json::to_string_pretty(&value).unwrap());

    // Check if the HTTP status is OK
    assert_eq!(status, Status::Ok);
}
