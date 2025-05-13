use rocket::http::Status;

use crate::tests::{
    test_client,
    users::{login, ADMIN_LOGIN},
    workspaces::{ROUTE_WORKSPACE, ROUTE_WORKSPACES, TARGETED_WORKSPACE},
};

#[test]
fn view_workspaces_from_user() {
    let client = test_client();

    // Log in
    login(&client, ADMIN_LOGIN);

    let response = client.get(format!("{ROUTE_WORKSPACES}")).dispatch();

    // Assert the login request was successful
    let status = response.status().clone();
    println!("{}", response.into_string().unwrap());
    assert_eq!(status, Status::Ok);
}

#[test]
fn view_workspace() {
    let client = test_client();

    // Log in
    login(&client, ADMIN_LOGIN);

    let response = client
        .get(format!("{ROUTE_WORKSPACE}{TARGETED_WORKSPACE}"))
        .dispatch();

    // Assert the login request was successful
    let status = response.status().clone();
    println!("{}", response.into_string().unwrap());
    assert_eq!(status, Status::Ok);
}
