use rocket::http::Status;

use crate::tests::{
    projects::ROUTE_PROJECTS,
    test_client,
    users::{login, ADMIN_LOGIN},
    workspaces::{ROUTE_WORKSPACES, TARGETED_WORKSPACE},
};

#[test]
fn view_projects_from_user() {
    let client = test_client();

    // Log in
    login(&client, ADMIN_LOGIN);

    let response = client
        .get(format!(
            "{ROUTE_WORKSPACES}{TARGETED_WORKSPACE}{ROUTE_PROJECTS}"
        ))
        .dispatch();

    // Assert the login request was successful
    let status = response.status().clone();
    println!("{}", response.into_string().unwrap());
    assert_eq!(status, Status::Ok);
}
