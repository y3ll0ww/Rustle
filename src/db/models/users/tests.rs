use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use rocket::{
    http::{ContentType, Status},
    local::blocking::Client,
};
use serde_json::json;

#[test]
fn create_new_user() {
    let client = Client::tracked(crate::rocket()).expect("valid rocket instance");

    // Use valid date values for created_at and updated_at fields
    let created_at = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    );
    let updated_at = created_at;

    // Construct a JSON payload matching the User structure
    let payload = json!({
        "user_id": "",  // Will be overwritten by `create_user` function with UUID
        "username": "y3ll0ww",
        "display_name": null,
        "email": "some@abc.nl",
        "password_hash": "password",
        "bio": null,
        "avatar_url": null,
        "created_at": created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),  // Convert to ISO 8601 format
        "updated_at": updated_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
    });

    // Send POST request to the correct endpoint `/users`
    let response = client
        .post("/users/create")
        .header(ContentType::JSON)
        .body(payload.to_string())
        .dispatch();

    // Assert that the response status is 200 (indicating success)
    assert_eq!(response.status(), Status::Ok);

    // Optionally, check the response body for success message
    let response_body = response.into_string().unwrap();
    println!("{response_body}");
    assert!(response_body.contains("User y3ll0ww created"));
}

#[test]
fn delete_user() {
    let client = Client::tracked(crate::rocket()).expect("valid rocket instance");

    // Send POST request to the correct endpoint `/users`
    let response = client
        .delete("/users/delete/15240908-e7a0-41cd-806e-39bd8b4d8b08")
        .dispatch();

    // Assert that the response status is 200 (indicating success)
    assert_eq!(response.status(), Status::Ok);
}