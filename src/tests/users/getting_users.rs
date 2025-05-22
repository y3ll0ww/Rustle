use rocket::http::{ContentType, Status};

use super::{login, DEFAULT_LOGIN, DEFAULT_USERNAME};
use crate::{
    database::pagination::{request::PaginationRequest, sort::UserField},
    tests::{
        test_client,
        users::{ADMIN_LOGIN, ROUTE_BROWSE, ROUTE_GET, ROUTE_GET_ALL},
    },
};

#[test]
fn get_all_users() {
    let client = test_client();

    // Login required
    login(&client, ADMIN_LOGIN);

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
fn browse_users() {
    let client = test_client();

    // Login required
    login(&client, DEFAULT_LOGIN);

    // Construct a JSON payload matching the User structure
    let params = PaginationRequest::<UserField> {
        page: Some(7),
        limit: Some(7),
        search: None,
        sort_by: None,
        sort_dir: None,
    };

    let payload = serde_json::to_string(&params).unwrap();

    let response = client
        .get(ROUTE_BROWSE)
        //.get(format!("{ROUTE_BROWSE}?status=0&role=0"))
        .header(ContentType::JSON)
        .body(payload)
        .dispatch();

    let status = response.status().clone();

    println!("{}", response.into_string().unwrap());

    assert_eq!(status, Status::Ok)
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
