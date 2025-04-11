use rocket::http::{ContentType, Status};

use crate::{
    forms::teams::NewTeamForm,
    routes::TEAMS,
    tests::{test_client, users::{login, DEFAULT_LOGIN}},
};

#[test]
fn new_team_by_form() {
    let client = test_client();

    // Log in
    login(&client, DEFAULT_LOGIN);

    // Create a form with test data
    let new_user = NewTeamForm {
        team_name: "Team name".to_string(),
        description: None,
    };

    // Send submit request
    let response = client
        .post(format!("{TEAMS}new"))
        .body(new_user.body())
        .header(ContentType::Form)
        .dispatch();

    // Assert the submit request was successful
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn view_all_teams_of_default_user() {
    let client = test_client();

    // Log in
    login(&client, DEFAULT_LOGIN);

    let response = client.get(format!("{TEAMS}")).dispatch();

    // Assert the login request was successful
    assert_eq!(response.status(), Status::Ok);

    println!("{}", response.into_string().unwrap());
}

#[test]
fn view_all_teams_without_logging_in() {
    let client = test_client();

    let response = client.get(format!("{TEAMS}")).dispatch();

    // Assert the login request was successful
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn view_team_of_default_user() {
    let client = test_client();

    let team_id = "a66a6bb3-9fe0-490c-ba4f-d14b2f18076e";

    // Log in
    login(&client, DEFAULT_LOGIN);

    let response = client.get(format!("{TEAMS}{team_id}")).dispatch();

    let status = response.status().clone();

    // Assert the login request was successful
    //assert_eq!(response.status(), Status::Ok);

    println!("{}", response.into_string().unwrap());
    assert_eq!(status, Status::Ok);
}

#[test]
fn delete_team_from_default_user() {
    let client = test_client();

    let team_id = "a66a6bb3-9fe0-490c-ba4f-d14b2f18076e";

    // Log in
    login(&client, DEFAULT_LOGIN);

    let response = client.delete(format!("{TEAMS}{team_id}/delete")).dispatch();

    let status = response.status().clone();

    // Assert the login request was successful
    //assert_eq!(response.status(), Status::Ok);

    println!("{}", response.into_string().unwrap());
    assert_eq!(status, Status::Ok);
}
