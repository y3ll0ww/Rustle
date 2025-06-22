use std::str::FromStr;

use rocket::http::ContentType;
use uuid::Uuid;

use crate::{
    models::projects::{ProjectMember, ProjectRole},
    tests::{
        projects::{route_projects_add_member, TARGETED_PROJECT},
        response_ok, test_client,
        users::{login, ADMIN_LOGIN},
    },
};

#[test]
fn add_member_to_project() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);
    let member_to_add = "41cb895a-cf97-4df4-b2d3-8479146086a8";

    // Define the information for the member to add
    let new_member = ProjectMember {
        project: Uuid::from_str(TARGETED_PROJECT).unwrap(),
        member: Uuid::from_str(member_to_add).unwrap(),
        role: i16::from(ProjectRole::Contributor),
    };

    // Serialize the workspace update
    let payload = serde_json::to_string(&[new_member]).unwrap();

    response_ok(
        client
            .post(route_projects_add_member())
            .header(ContentType::JSON)
            .body(payload),
    );
}
