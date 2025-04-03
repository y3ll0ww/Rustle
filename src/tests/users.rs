use std::str::FromStr;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

use rocket::{
    http::{ContentType, Status},
    local::blocking::{Client, LocalResponse},
};
use uuid::Uuid;

use crate::{
    cookies::TOKEN_COOKIE,
    forms::users::{LoginForm, NewUserForm, Password},
    models::users::{User, UserRole, UserStatus},
    tests::test_client,
};

const USERNAME: &str = "test_user";
const PASSWORD: &str = "strong_password";

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
        username: "y3ll0ww".to_string(),
        display_name: None,
        email: "some@abc.nl".to_string(),
        password: "password".to_string(),
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
    assert!(response_body.contains("User y3ll0ww created"));
}

/// Creates a new user in the database using form data.
///
/// It will submit a [`NewUserForm`] and check the response status code. Then it will validate the
/// cookies to check if the newly created user is logged in.
///
/// ## Prerequisites
///
/// Make sure there is no standard user already added to the database or the test will fail.
#[test]
fn submit_new_user_by_form() {
    let client = test_client();

    // Create a form with test data
    let new_user = NewUserForm {
        username: USERNAME,
        password: Password {
            first: PASSWORD,
            second: PASSWORD,
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

/// Logging in and logging out a user.
///
/// This test will log in the standard user and verify if the token and user information is
/// correctly added to the cookies. Then it will log out the standard user and verify that said
/// information is removed from the cookies.
///
/// ## Prerequisites
///
/// There should already be a standard user added to the database. This can be done by running the
/// test `test_submit`.
#[test]
fn login_existing_user_then_logout() {
    let client = test_client();

    // Log in
    default_login(&client);

    // Log out
    let logout_response = client.post("/user/logout").dispatch();

    // Assert that the logout request was handled succesfully
    assert_eq!(logout_response.status(), Status::Ok);

    // Assert that the cookies are removed
    assert_authorized_cookies(logout_response, false);
}

#[test]
fn logout_without_being_logged_in() {
    let client = test_client();

    // Log out
    let logout_response = client.post("/user/logout").dispatch();

    // Assert that the logout request returned "Unauthorized"
    assert_eq!(logout_response.status(), Status::Unauthorized);
}

/// Removes a user with a given user ID from the database.
///
/// ## Prerequisites
///
/// The user should already exist in the database. This can be done by running the test
/// `test_submit`.
///
/// The tester should know the user ID from the desired user, then replace the `user_id` variable
/// inside the test accordingly.
#[test]
fn delete_existing_user_by_id() {
    let client = test_client();

    // Login required
    default_login(&client);

    // User ID: Change depending on which user tester wants to delete
    let user_id = "1fca2643-ec64-488d-b822-fe85b489114e";

    // Send delete request
    let response = client.delete(format!("/user/{user_id}/delete")).dispatch();

    // Assert the delete request was successful
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn get_user_by_username() {
    let client = test_client();

    // Login required
    default_login(&client);

    // Send get request
    let response = client.get(format!("/user/{USERNAME}")).dispatch();

    // Copy the status for later assertion
    let status = response.status().clone();

    // Extract the data to print it to the screen
    let data = response.into_string();

    // Assert the delete request was successful
    assert_eq!(status, Status::Ok);

    // Print the data to the screen
    assert!(data.is_some());
    println!("{:?}", data.unwrap());
}

#[test]
fn get_all_users() {
    let client = test_client();

    // Login required
    default_login(&client);

    // Send get request
    let response = client.get(format!("/user")).dispatch();

    // Copy the status for later assertion
    let status = response.status().clone();

    // Extract the data to print it to the screen
    let data = response.into_string();

    // Assert the delete request was successful
    assert_eq!(status, Status::Ok);

    // Print the data to the screen
    assert!(data.is_some());
    println!("{:?}", data.unwrap());
}

pub fn default_login(client: &Client) {
    // Create a form with test data
    let login = LoginForm {
        username: USERNAME,
        password: PASSWORD,
    };

    let login_response = client
        .post("/user/login")
        .header(ContentType::Form)
        .body(login.body())
        .dispatch();

    // Assert the login request was successful
    assert_eq!(login_response.status(), Status::Ok);

    // Assert that the cookies are added
    assert_authorized_cookies(login_response, true);
}

fn assert_authorized_cookies(response: LocalResponse<'_>, available: bool) {
    // Get the cookies after the response
    let cookies = response.cookies();
    let token_cookie = cookies.get_private(TOKEN_COOKIE);

    // Perform the assertions on the cookies based on provided boolean
    if available {
        assert!(token_cookie.is_some());
    } else {
        assert!(token_cookie.is_none());
    }
}
