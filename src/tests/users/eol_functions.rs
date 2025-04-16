use std::str::FromStr;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rocket::http::{ContentType, Status};
use uuid::Uuid;

use super::{assert_authorized_cookies, ADMIN_PASSWORD, ADMIN_USERNAME, DEFAULT_PASSWORD, DEFAULT_USERNAME};
use crate::{
    forms::users::{NewUserForm, Password},
    models::users::{User, UserRole, UserStatus},
    tests::test_client,
};

#[test]
fn inject_admin_user() {
    let client = test_client();

    // Use valid date values for created_at and updated_at fields
    let created_at = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    );
    let updated_at = DateTime::from_timestamp(Utc::now().timestamp(), 0)
        .unwrap()
        .naive_utc();

    // Construct a JSON payload matching the User structure
    let user = User {
        id: Uuid::from_str("a99b50c6-02e9-4142-95fe-35c3ccd4f147").unwrap(),
        role: i16::from(UserRole::Admin),
        status: i16::from(UserStatus::Active),
        username: ADMIN_USERNAME.to_string(),
        display_name: None,
        email: "some@abc.nl".to_string(),
        password: Password::generate(Some(ADMIN_PASSWORD)).unwrap(),
        bio: None,
        avatar_url: None,
        created_at,
        updated_at,
    };

    let payload = serde_json::to_string(&user).unwrap();

    // Send POST request to the correct endpoint `/users`
    let response = client
        .post("/user/create")
        .header(ContentType::JSON)
        .body(payload)
        .dispatch();

    // Assert that the response status is 200 (indicating success)
    assert_eq!(response.status(), Status::Ok);

    // Optionally, check the response body for success message
    let response_body = response.into_string().unwrap();
    println!("{response_body}");
    assert!(response_body.contains(&format!("User {ADMIN_USERNAME} created")));
}

#[test]
fn submit_new_user_by_form() {
    let client = test_client();

    // Create a form with test data
    let new_user = NewUserForm {
        username: DEFAULT_USERNAME,
        password: Password {
            first: DEFAULT_PASSWORD,
            second: DEFAULT_PASSWORD,
        },
        email: "test@example.com",
    };

    // Send submit request
    let response = client
        .post("/user/register")
        .body(new_user.body()) // Use the formatted string as the body
        .header(ContentType::Form)
        .dispatch();

    // Assert the submit request was successful
    assert_eq!(response.status(), Status::Ok);

    // Assert that the cookies are added
    assert_authorized_cookies(response, true);
}
