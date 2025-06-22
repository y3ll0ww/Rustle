use crate::tests::{
    projects::{route_get_projects_by_user, route_get_projects_from_workspace, route_projects_get},
    response_ok, test_client,
    users::{login, ADMIN_LOGIN, DEFAULT_LOGIN},
};

#[test]
fn view_projects_from_workspace() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);
    response_ok(client.get(route_get_projects_from_workspace()));
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
