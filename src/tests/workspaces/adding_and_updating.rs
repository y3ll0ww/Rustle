use rocket::http::ContentType;

use crate::{
    forms::workspace::NewWorkspaceForm,
    models::workspaces::WorkspaceUpdate,
    tests::{
        response_ok, test_client,
        users::{login, ADMIN_LOGIN},
        workspaces::{route_workspaces_new, route_workspaces_update},
    },
};

#[test]
fn new_workspace_by_form() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);

    // Create a form with test data
    let new_workspace = NewWorkspaceForm {
        name: "New Workspace".to_string(),
        description: None,
    };

    response_ok(
        client
            .post(route_workspaces_new())
            .header(ContentType::Form)
            .body(new_workspace.body()),
    );
}

#[test]
fn update_workspace_information() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);

    // Define the information to update
    let workspace_update = WorkspaceUpdate {
        name: None,
        description: Some("I'm adding some workspace description here.".to_string()),
        image_url: None,
    };

    // Serialize the workspace update
    let payload = serde_json::to_string(&workspace_update).unwrap();

    response_ok(
        client
            .put(route_workspaces_update())
            .header(ContentType::JSON)
            .body(payload),
    );
}
