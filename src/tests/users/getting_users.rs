use rocket::http::Status;

use super::{login, DEFAULT_LOGIN, DEFAULT_USERNAME};
use crate::tests::{
    test_client,
    users::{ROUTE_GET, ROUTE_GET_ALL},
};

#[test]
fn get_all_users() {
    let client = test_client();

    // Login required
    login(&client, DEFAULT_LOGIN);

    // Send get request
    let response = client.get(ROUTE_GET_ALL).dispatch();

    // Copy the status for later assertion
    let status = response.status().clone();

    // Extract the data to print it to the screen
    let data = response.into_string();

    // Assert the delete request was successful
    assert_eq!(status, Status::Ok);

    // Print the data to the screen
    assert!(data.is_some());
    println!("{:?}", data.unwrap());
}

#[test]
fn get_user_by_username() {
    let client = test_client();

    // Login required
    login(&client, DEFAULT_LOGIN);

    // Send get request
    let response = client
        .get(format!("{ROUTE_GET}{DEFAULT_USERNAME}"))
        .dispatch();

    // Copy the status for later assertion
    let status = response.status().clone();

    // Extract the data to print it to the screen
    let data = response.into_string();

    // Assert the delete request was successful
    assert_eq!(status, Status::Ok);

    // Print the data to the screen
    assert!(data.is_some());
    println!("{:?}", data.unwrap());
}
