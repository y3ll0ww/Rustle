use crate::tests::{
    projects::{ROUTE_PROJECTS, ROUTE_PROJECTS_, TARGETED_PROJECT},
    response_ok, test_client,
    users::{login, ADMIN_LOGIN},
    workspaces::{ROUTE_WORKSPACES, TARGETED_WORKSPACE},
};

#[test]
fn view_projects_from_user() {
    let client = test_client();

    // Log in
    login(&client, ADMIN_LOGIN);

    // Fetch projects from workspace
    response_ok(
        client
            .get(format!(
                "{ROUTE_WORKSPACES}{TARGETED_WORKSPACE}{ROUTE_PROJECTS}"
            ))
            .dispatch(),
    );
}

#[test]
fn view_project_by_id() {
    let client = test_client();

    // Log in
    login(&client, ADMIN_LOGIN);

    // Fetch project with its ID
    response_ok(
        client
            .get(format!(
                "{ROUTE_PROJECTS_}{TARGETED_PROJECT}"
            ))
            .dispatch(),
    );
}
