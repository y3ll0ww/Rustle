use rocket::http::ContentType;
use uuid::Uuid;

use super::{login, DEFAULT_LOGIN, DEFAULT_USERNAME};
use crate::{
    database::pagination::{request::PaginationRequest, sort::UserField},
    tests::{
        response_ok, test_client,
        users::{route_users_browse, route_users_by_name, ADMIN_LOGIN},
        workspaces::TARGETED_WORKSPACE,
    },
};

#[test]
fn browse_users() {
    let client = test_client();
    login(&client, ADMIN_LOGIN);

    // Apply filters
    let status: Option<u16> = None;
    let role: Option<u16> = None;
    let workspace: Option<Uuid> = Some(Uuid::try_from(TARGETED_WORKSPACE).unwrap());
    let exclude_self: bool = false;

    // Construct a JSON payload matching the User structure
    let params = PaginationRequest::<UserField> {
        page: Some(7),
        limit: Some(7),
        search: Some("rustle".to_string()),
        sort_by: None,
        sort_dir: None,
    };

    // Define the payload
    let payload = serde_json::to_string(&params).unwrap();

    response_ok(
        client
            .post(route_users_browse(status, role, workspace, exclude_self))
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
