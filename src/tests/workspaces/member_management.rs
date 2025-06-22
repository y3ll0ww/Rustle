use std::str::FromStr;

use rocket::http::{ContentType, Status};
use uuid::Uuid;

use crate::{
    models::workspaces::{WorkspaceMember, WorkspaceRole},
    tests::{
        test_client,
        users::{login, ADMIN_LOGIN},
        workspaces::{ROUTE_WORKSPACES, TARGETED_WORKSPACE},
    },
};

#[test]
fn add_member_to_workspace() {
    let client = test_client();

    let member_to_add = "41cb895a-cf97-4df4-b2d3-8479146086a8";

    // Define the information for the member to add
    let new_member = WorkspaceMember {
        workspace: Uuid::from_str(TARGETED_WORKSPACE).unwrap(),
        member: Uuid::from_str(member_to_add).unwrap(),
        role: i16::from(WorkspaceRole::Contributor),
    };

    // Serialize the workspace update
    let payload = serde_json::to_string(&[new_member]).unwrap();

    // Log in as user with sufficient status
    login(&client, ADMIN_LOGIN);

    // Send the request
    let response = client
        .post(format!(
            "{ROUTE_WORKSPACES}{TARGETED_WORKSPACE}/add-members"
        ))
        .header(ContentType::JSON)
        .body(payload)
        .dispatch();

    // Assert the request was successful
    let status = response.status().clone();
    println!("{:?}", response.into_string());
    assert_eq!(status, Status::Ok);
}

#[test]
fn remove_member_from_workspace() {
    let client = test_client();

    let member_to_remove = "41cb895a-cf97-4df4-b2d3-8479146086a8";

    // Log in as user with sufficient status
    login(&client, ADMIN_LOGIN);

    // Send the request
    let response = client
        .delete(format!(
            "{ROUTE_WORKSPACES}{TARGETED_WORKSPACE}/remove-member/{member_to_remove}"
        ))
        .dispatch();

    // Assert the request was successful
    let status = response.status().clone();
    println!("{:?}", response.into_string());
    assert_eq!(status, Status::Ok);
}
