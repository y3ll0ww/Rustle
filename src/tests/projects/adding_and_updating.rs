use rocket::http::ContentType;

use crate::{
    forms::projects::NewProjectForm,
    tests::{
        projects::route_projects_create,
        response_ok, test_client,
        users::{login, ADMIN_LOGIN},
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
    response_ok(
        client
            .post(route_projects_create())
            .body(new_project.body())
            .header(ContentType::Form)
            .dispatch(),
    );
}
