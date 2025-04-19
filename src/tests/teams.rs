use rocket::http::{ContentType, Status};

use crate::{
    forms::workspace::NewWorkspaceForm, models::workspaces::WorkspaceUpdate, routes::WORKSPACES, tests::{
        test_client,
        users::{login, ADMIN_LOGIN, DEFAULT_LOGIN},
    }
};

const ROUTE_WORKSPACE_NEW: &str = "/workspaces/new";

#[test]
fn new_workspace_by_form() {
    let client = test_client();

    // Log in
    login(&client, ADMIN_LOGIN);

    // Create a form with test data
    let new_user = NewWorkspaceForm {
        name: "New Workspace".to_string(),
        description: None,
    };

    // Send submit request
    let response = client
        .post(ROUTE_WORKSPACE_NEW)
        .body(new_user.body())
        .header(ContentType::Form)
        .dispatch();

    // Assert the submit request was successful
    let status = response.status().clone();
    println!("{:?}", response.into_string());
    assert_eq!(status, Status::Ok);
}

#[test]
fn update_basic_workspace_information() {
    let client = test_client();

    // Define the workspace ID to update
    let workspace_to_update = "4a9860c4-1cbc-4e77-a6dd-1e917f2de243";

    // Define the information to update
    let workspace_update = WorkspaceUpdate {
        name: None,
        description: Some("I'm adding some workspace description here.".to_string()),
        image_url: None,
    };

    // Serialize the workspace update
    let payload = serde_json::to_string(&workspace_update).unwrap();

    // Log in
    login(&client, ADMIN_LOGIN);

    // Send the request
    let response = client
        .put(format!("/workspaces/{workspace_to_update}/update"))
        .header(ContentType::JSON)
        .body(payload)
        .dispatch();

    // Assert the request was successful
    let status = response.status().clone();
    println!("{:?}", response.into_string());
    assert_eq!(status, Status::Ok);
}

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
