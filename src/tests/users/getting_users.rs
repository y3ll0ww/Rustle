use rocket::http::ContentType;

use super::{login, DEFAULT_LOGIN, DEFAULT_USERNAME};
use crate::{
    database::pagination::{request::PaginationRequest, sort::UserField},
    tests::{
        response_ok, test_client,
        users::{route_users_all, route_users_browse, route_users_by_name, ADMIN_LOGIN},
    },
};

#[test]
fn get_all_users() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);
    response_ok(client.get(route_users_all()));
}

#[test]
fn browse_users() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);

    // Apply filters
    let status: Option<u16> = None;
    let role: Option<u16> = None;

    // Construct a JSON payload matching the User structure
    let params = PaginationRequest::<UserField> {
        page: Some(7),
        limit: Some(7),
        search: Some("example".to_string()),
        sort_by: None,
        sort_dir: None,
    };

    // Define the payload
    let payload = serde_json::to_string(&params).unwrap();

    response_ok(
        client
            .get(route_users_browse(status, role))
            .header(ContentType::JSON)
            .body(payload),
    );
}

#[test]
fn get_user_by_username() {
    let client = test_client();
    login(&client, DEFAULT_LOGIN);
    response_ok(client.get(route_users_by_name(DEFAULT_USERNAME)));
}
