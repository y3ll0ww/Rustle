use rocket::http::{ContentType, Status};

use crate::{
    forms::workspace::NewWorkspaceForm,
    models::workspaces::WorkspaceUpdate,
    tests::{
        test_client,
        users::{login, ADMIN_LOGIN},
        workspaces::{ROUTE_WORKSPACE, ROUTE_WORKSPACE_NEW},
    },
};

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
fn update_workspace_information() {
    let client = test_client();

    // Define the workspace ID to update
    let workspace_to_update = "2e06634d-2da3-44cb-9c81-326b6715efce";

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
        .put(format!("{ROUTE_WORKSPACE}/{workspace_to_update}/update"))
        .header(ContentType::JSON)
        .body(payload)
        .dispatch();

    // Assert the request was successful
    let status = response.status().clone();
    println!("{:?}", response.into_string());
    assert_eq!(status, Status::Ok);
}
