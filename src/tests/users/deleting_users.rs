use rocket::http::Status;

use super::{login, ADMIN_LOGIN, ADMIN_USERNAME, DEFAULT_USERNAME};
use crate::tests::{
    test_client,
    users::{ROUTE_DELETE, ROUTE_GET_ALL},
};

#[test]
fn delete_existing_user_by_id() {
    let client = test_client();

    // Login required
    login(&client, ADMIN_LOGIN);

    // User ID: Change depending on which user tester wants to delete
    let user_id = "77987439-2fed-4d45-9f5d-4c02c66eb265";

    // Send delete request
    let response = client.delete(format!("{ROUTE_DELETE}{user_id}")).dispatch();

    // Assert the delete request was successful
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn delete_all_users_except_for_admin_and_test_user() {
    let client = test_client();

    // Login required
    login(&client, ADMIN_LOGIN);

    // Get all the users
    let response = client.get(ROUTE_GET_ALL).dispatch();

    // Convert the response to a Value
    let response_value: serde_json::Value =
        serde_json::from_str(&response.into_string().unwrap()).unwrap();

    // Extract the "data" field from the response
    let data = serde_json::to_value(response_value)
        .unwrap()
        .get("data")
        .unwrap()
        .clone();

    // Check if the data is an array
    if let serde_json::Value::Array(users) = data {
        // Iterate over the users and delete them
        for user in users {
            // Get the user ID and username
            let user_id = user.get("id").and_then(|v| v.as_str()).unwrap();
            let username = user.get("username").and_then(|v| v.as_str()).unwrap();

            // Skip the admin and test user
            if username == ADMIN_USERNAME || username == DEFAULT_USERNAME {
                continue;
            }

            // Delete the user
            let response = client.delete(format!("{ROUTE_DELETE}{user_id}")).dispatch();
            assert_eq!(response.status(), Status::Ok);
        }
    }

    // Assert that the delete request was successful
    let response = client.get(ROUTE_GET_ALL).dispatch();
    let status = response.status().clone();

    println!("{:?}", response.into_string());
    assert_eq!(status, Status::Ok);
}
