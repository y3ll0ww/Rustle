use rocket::http::Status;

use super::{login, ADMIN_LOGIN, ADMIN_USERNAME, DEFAULT_USERNAME};
use crate::tests::{
    response_ok, test_client,
    users::{route_users_all, route_users_delete},
};

const TARGETED_USER: &str = "77987439-2fed-4d45-9f5d-4c02c66eb265";

#[test]
fn delete_existing_user_by_id() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);
    response_ok(client.delete(route_users_delete(TARGETED_USER)));
}

#[test]
fn delete_all_users_except_for_admin_and_test_user() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);

    // Get all the users
    let response = client.get(route_users_all()).dispatch();

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
            let response = client.delete(route_users_delete(user_id)).dispatch();
            assert_eq!(response.status(), Status::Ok);
        }
    }

    // Assert that the delete request was successful
    response_ok(client.get(route_users_all()));
}
