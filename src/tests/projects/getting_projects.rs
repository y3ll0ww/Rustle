use rocket::http::ContentType;

use crate::{
    database::pagination::{request::PaginationRequest, sort::ProjectField},
    tests::{
        projects::{
            route_get_projects_by_user, route_get_projects_from_workspace,
            route_get_projects_paginated, route_projects_get,
        },
        response_ok, test_client,
        users::{login, ADMIN_LOGIN, DEFAULT_LOGIN},
        workspaces::TARGETED_WORKSPACE,
    },
};

#[test]
fn view_projects_from_workspace() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);
    response_ok(client.get(route_get_projects_from_workspace()));
}

#[test]
fn view_projects_from_workspace_paginated() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);

    // Apply filters
    let workspace: Option<&str> = Some(TARGETED_WORKSPACE);
    let user: Option<&str> = None;

    // Construct a JSON payload matching the User structure
    let params = PaginationRequest::<ProjectField> {
        page: None,
        limit: Some(10),
        search: None,
        sort_by: None,
        sort_dir: None,
    };

    // Define the payload
    let payload = serde_json::to_string(&params).unwrap();

    response_ok(
        client
            .get(route_get_projects_paginated(workspace, user))
            .header(ContentType::JSON)
            .body(payload),
    );
}

#[test]
fn view_projects_by_user() {
    let client = test_client();
    login(&client, DEFAULT_LOGIN);
    response_ok(client.get(route_get_projects_by_user()));
}

#[test]
fn view_project_by_id() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);
    response_ok(client.get(route_projects_get()));
}
