use rocket::{
    http::Status,
    local::{
        asynchronous::Client as AsyncClient,
        blocking::{Client, LocalRequest},
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

fn root_route(base: &str) -> String {
    let mut route = base.to_string();
    route.pop(); // Remove the tailing slash; it will invalidate the endpoint
    route
}

pub fn response_ok(request: LocalRequest<'_>) {
    response_status(request, Status::Ok)
}

pub fn response_not_found(request: LocalRequest<'_>) {
    response_status(request, Status::NotFound)
}

pub fn response_unauthorized(request: LocalRequest<'_>) {
    response_status(request, Status::Unauthorized)
}

fn response_status(request: LocalRequest<'_>, expected_status: Status) {
    // Get the response
    let response = request.dispatch();

    // Extract the status
    let status = response.status().clone();

    // Format and print the response
    let body = response.into_string().unwrap_or("{}".to_string());

    let debug = match serde_json::from_str::<Value>(&body) {
        Ok(value) => serde_json::to_string_pretty(&value).unwrap(),
        Err(e) => e.to_string(),        
    };

    println!("{debug}");

    // Check if the HTTP status is as expected
    assert_eq!(status, expected_status);
}
