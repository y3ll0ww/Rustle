use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use rocket::{
    http::{ContentType, Header, Status},
    local::blocking::Client,
};
use serde_json::json;

use crate::{
    api::ApiResponse,
    forms::users::{InsertedUser, NewUserForm, Password},
};

const USERNAME: &str = "test_user";
const PASSWORD: &str = "strong_password";

fn test_client() -> Client {
    Client::tracked(crate::rocket()).expect("valid rocket instance")
}

#[test]
fn create_new_user() {
    let client = test_client();

    // Use valid date values for created_at and updated_at fields
    let created_at = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    );
    let updated_at = created_at;

    // Construct a JSON payload matching the User structure
    let payload = json!({
        "id": "a99b50c6-02e9-4142-95fe-35c3ccd4f147",  // Will be overwritten by `create_user` function with UUID
        "privilege": 3,
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
        .post("/user/create")
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
    let client = test_client();

    // Create a form with test data
    let new_user = NewUserForm {
        username: USERNAME,
        display_name: Some("Test User"),
        password: Password {
            first: PASSWORD,
            second: PASSWORD,
        },
        email: "test@example.com",
    };

    // Format the form data manually
    let display_name = new_user.display_name.unwrap_or("");

    let f = format!(
        "username={}&display_name={}&password.first={}&password.second={}&email={}",
        new_user.username,
        display_name,
        new_user.password.first,  // Use only the first password for the field
        new_user.password.second, // Include the second password field as well
        new_user.email,
    );

    // Send a POST request to the /form route with the form data
    let response = client
        .post("/user/form")
        .body(f) // Use the formatted string as the body
        .header(ContentType::Form)
        .dispatch();

    // Assert the response status
    assert_eq!(response.status(), Status::Ok);

    // Assert that the return time is InstertedUser
    let api_response =
        serde_json::from_str::<ApiResponse<InsertedUser>>(&response.into_string().unwrap())
            .unwrap();

    assert_eq!(api_response.data.unwrap().username, new_user.username);
}

#[test]
fn delete_user() {
    let client = test_client();

    // Send POST request to the correct endpoint `/user`
    let response = client
        .delete("/user/delete/0772e26b-0569-44e3-b291-033356794047")
        .dispatch();

    // Assert that the response status is 200 (indicating success)
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn login_logout_user() {
    let client = test_client();

    // Login the test user
    let login_response = client
        .post("/user/login")
        .header(ContentType::Form)
        .body(format!("username={USERNAME}&password={PASSWORD}"))
        .dispatch();

    assert_eq!(login_response.status(), Status::Ok);

    let token: String = login_response
        .into_json::<ApiResponse<String>>()
        .unwrap()
        .data
        .unwrap();

    let logout_response = client
        .post("/user/logout")
        .header(Header::new("Authorization", format!("Bearer {token}")))
        .dispatch();

    assert_eq!(logout_response.status(), Status::Ok);
}
