use crate::tests::{
    response_ok, test_client,
    users::{login, ADMIN_LOGIN, DEFAULT_LOGIN},
    workspaces::{ROUTE_WORKSPACES, ROUTE_WORKSPACES_LIST, TARGETED_WORKSPACE},
};

#[test]
fn view_workspaces_from_user() {
    let client = test_client();
    login(&client, DEFAULT_LOGIN);
    response_ok(client.get(ROUTE_WORKSPACES_LIST));
}

#[test]
fn view_workspace() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);
    response_ok(client.get(format!("{ROUTE_WORKSPACES}{TARGETED_WORKSPACE}")));
}
