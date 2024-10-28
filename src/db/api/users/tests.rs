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
fn test_submit() {
    let client = Client::tracked(crate::rocket()).expect("valid rocket instance");

    // Create a form with test data
    let account = crate::users::form::Account {
        username: "test_user",
        display_name: Some("Test User"),
        password: crate::users::form::Password {
            first: "strong_password",
            second: "strong_password",
        },
        email: "test@example.com",
        bio: Some("This is a test bio."),
        avatar_url: Some("http://example.com/avatar.png"),
    };

    // Format the form data manually
    let display_name = account.display_name.unwrap_or("");
    let bio = account.bio.unwrap_or("");
    let avatar_url = account.avatar_url.unwrap_or("");

    let f = format!(
        "username={}&display_name={}&password.first={}&password.second={}&email={}&bio={}&avatar_url={}",
        account.username,
        display_name,
        account.password.first, // Use only the first password for the field
        account.password.second, // Include the second password field as well
        account.email,
        bio,
        avatar_url,
    );

    // Send a POST request to the /form route with the form data
    let response = client
        .post("/users/form")
        .body(f) // Use the formatted string as the body
        .header(ContentType::Form)
        .dispatch();

    // Assert the response status and body
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "User test_user created");
}

#[test]
fn delete_user() {
    let client = Client::tracked(crate::rocket()).expect("valid rocket instance");

    // Send POST request to the correct endpoint `/users`
    let response = client
        .delete("/users/delete/a99b50c6-02e9-4142-95fe-35c3ccd4f147")
        .dispatch();

    // Assert that the response status is 200 (indicating success)
    assert_eq!(response.status(), Status::Ok);
}
