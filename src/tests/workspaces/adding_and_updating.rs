use std::str::FromStr;

use rocket::http::{ContentType, Status};
use uuid::Uuid;

use crate::{
    forms::workspace::NewWorkspaceForm,
    models::workspaces::{WorkspaceMember, WorkspaceRole, WorkspaceUpdate},
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

#[test]
fn add_member_to_workspace() {
    let client = test_client();

    // Define the workspace ID to update
    let workspace_to_update = "2e06634d-2da3-44cb-9c81-326b6715efce";
    let member_to_add = "41cb895a-cf97-4df4-b2d3-8479146086a8";

    // Define the information for the member to add
    let new_member = WorkspaceMember {
        workspace: Uuid::from_str(workspace_to_update).unwrap(),
        member: Uuid::from_str(member_to_add).unwrap(),
        role: i16::from(WorkspaceRole::Contributor),
    };

    // Serialize the workspace update
    let payload = serde_json::to_string(&[new_member]).unwrap();

    // Log in as user with sufficient status
    login(&client, ADMIN_LOGIN);

    // Send the request
    let response = client
        .post(format!("{ROUTE_WORKSPACE}{workspace_to_update}/add-members"))
        .header(ContentType::JSON)
        .body(payload)
        .dispatch();

    // Assert the request was successful
    let status = response.status().clone();
    println!("{:?}", response.into_string());
    assert_eq!(status, Status::Ok);
}