use rocket::http::Status;

use crate::tests::{test_client, users::{assert_authorized_cookies, login, DEFAULT_LOGIN}};

#[test]
fn login_existing_user_then_logout() {
    let client = test_client();

    // Log in
    login(&client, DEFAULT_LOGIN);

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
