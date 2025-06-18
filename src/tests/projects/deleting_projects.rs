use crate::tests::{
    projects::route_projects_delete,
    response_ok, test_client,
    users::{login, ADMIN_LOGIN},
};

#[test]
fn delete_existing_project_by_id() {
    let client = test_client();

    // Login required
    login(&client, ADMIN_LOGIN);

    response_ok(client.delete(route_projects_delete()).dispatch());
}
