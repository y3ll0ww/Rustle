use rocket::http::Status;

use crate::tests::{test_client, users::default_login};

#[test]
fn view_all_teams_of_default_user() {
    let client = test_client();

    // Log in
    default_login(&client);

    let response = client
        .get("/team/overview")
        .dispatch();

    // Assert the login request was successful
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn view_all_teams_without_logging_in() {
    let client = test_client();

    let response = client
        .get("/team/overview")
        .dispatch();

    // Assert the login request was successful
    assert_eq!(response.status(), Status::Unauthorized);
}