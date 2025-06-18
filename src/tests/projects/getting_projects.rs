use crate::tests::{
    projects::{route_get_projects_from_workspace, route_projects_get},
    response_ok, test_client,
    users::{login, ADMIN_LOGIN},
};

#[test]
fn view_projects_from_user() {
    let client = test_client();

    // Log in
    login(&client, ADMIN_LOGIN);

    // Fetch projects from workspace
    response_ok(client.get(route_get_projects_from_workspace()).dispatch());
}

#[test]
fn view_project_by_id() {
    let client = test_client();

    // Log in
    login(&client, ADMIN_LOGIN);

    // Fetch project with its ID
    response_ok(client.get(route_projects_get()).dispatch());
}
