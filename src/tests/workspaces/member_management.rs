use std::str::FromStr;

use rocket::http::ContentType;
use uuid::Uuid;

use crate::{
    models::workspaces::{WorkspaceMember, WorkspaceRole},
    tests::{
        response_ok, test_client,
        users::{login, ADMIN_LOGIN},
        workspaces::{
            route_workspaces_add_member, route_workspaces_remove_member, TARGETED_WORKSPACE,
        },
    },
};

const TARGETED_MEMBER: &str = "41cb895a-cf97-4df4-b2d3-8479146086a8";

#[test]
fn add_member_to_workspace() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);

    // Define the information for the member to add
    let new_member = WorkspaceMember {
        workspace: Uuid::from_str(TARGETED_WORKSPACE).unwrap(),
        member: Uuid::from_str(TARGETED_MEMBER).unwrap(),
        role: i16::from(WorkspaceRole::Contributor),
    };

    // Serialize the workspace update
    let payload = serde_json::to_string(&[new_member]).unwrap();

    response_ok(
        client
            .post(route_workspaces_add_member())
            .header(ContentType::JSON)
            .body(payload),
    );
}

#[test]
fn remove_member_from_workspace() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);
    response_ok(client.delete(route_workspaces_remove_member(TARGETED_MEMBER)));
}
