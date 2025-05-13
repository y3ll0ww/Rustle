use rocket::http::Status;

use crate::{
    routes::WORKSPACES, tests::{
        test_client,
        users::{login, DEFAULT_LOGIN},
    }
};

#[cfg(test)]
mod adding_and_updating;
#[cfg(test)]
mod getting_workspaces;
#[cfg(test)]
mod member_management;

const ROUTE_WORKSPACE: &str = "/workspaces/";
const ROUTE_WORKSPACES: &str = "/workspaces";
const ROUTE_WORKSPACE_NEW: &str = "/workspaces/new";

pub const TARGETED_WORKSPACE: &str = "ad5d4bf9-2e80-47b0-8454-1c431718b666";

#[test]
fn view_all_teams_of_default_user() {
    let client = test_client();

    // Log in
    login(&client, DEFAULT_LOGIN);

    let response = client.get(format!("{WORKSPACES}")).dispatch();

    // Assert the login request was successful
    assert_eq!(response.status(), Status::Ok);

    println!("{}", response.into_string().unwrap());
}

#[test]
fn view_all_teams_without_logging_in() {
    let client = test_client();

    let response = client.get(format!("{WORKSPACES}")).dispatch();

    // Assert the login request was successful
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn view_team_of_default_user() {
    let client = test_client();

    let team_id = "a66a6bb3-9fe0-490c-ba4f-d14b2f18076e";

    // Log in
    login(&client, DEFAULT_LOGIN);

    let response = client.get(format!("{WORKSPACES}{team_id}")).dispatch();

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

    let response = client
        .delete(format!("{WORKSPACES}{team_id}/delete"))
        .dispatch();

    let status = response.status().clone();

    // Assert the login request was successful
    //assert_eq!(response.status(), Status::Ok);

    println!("{}", response.into_string().unwrap());
    assert_eq!(status, Status::Ok);
}
