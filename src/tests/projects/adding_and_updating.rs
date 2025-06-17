use rocket::http::{ContentType, Status};

use crate::{
    forms::projects::NewProjectForm,
    tests::{
        projects::ROUTE_PROJECTS_NEW,
        test_client,
        users::{login, ADMIN_LOGIN},
        workspaces::TARGETED_WORKSPACE,
    },
};

#[test]
fn new_project_by_form() {
    let client = test_client();

    // Log in
    login(&client, ADMIN_LOGIN);

    // Create a form with test data
    let new_project = NewProjectForm {
        name: "New Project".to_string(),
        description: None,
    };

    // Send submit request
    let response = client
        .post(format!("{ROUTE_PROJECTS_NEW}{TARGETED_WORKSPACE}"))
        .body(new_project.body())
        .header(ContentType::Form)
        .dispatch();

    // Assert the submit request was successful
    let status = response.status().clone();
    println!("{:?}", response.into_string());
    assert_eq!(status, Status::Ok);
}
