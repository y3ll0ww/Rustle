use crate::tests::{
    response_ok, test_client,
    users::{login, ADMIN_LOGIN, DEFAULT_LOGIN},
    workspaces::{route_workspaces_all, route_workspaces_by_id},
};

#[test]
fn view_workspaces_from_user() {
    let client = test_client();
    login(&client, DEFAULT_LOGIN);
    response_ok(client.get(route_workspaces_all()));
}

#[test]
fn view_workspace() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);
    response_ok(client.get(route_workspaces_by_id()));
}
