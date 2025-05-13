use std::str::FromStr;

use rocket::{
    http::{ContentType, Status},
    local::asynchronous::Client,
    State,
};
use uuid::Uuid;

use super::{async_login, DEFAULT_LOGIN, DEFAULT_PASSWORD};
use crate::{
    api::ApiResponse,
    cache::{self, RedisMutex},
    forms::{
        invite::{InvitedMultipleUsersForm, InvitedUserForm},
        password::Password,
    },
    models::{users::{PublicUser, UserStatus}, workspaces::WorkspaceRole},
    tests::{
        async_test_client,
        users::{
            ADMIN_LOGIN, DUPLICATE_USER_1_EMAIL_ADDR, DUPLICATE_USER_2_EMAIL_ADDR,
            INVITED_USER_1_EMAIL_ADDR, INVITED_USER_1_FIRST_NAME, INVITED_USER_1_LAST_NAME,
            INVITED_USER_1_LOGIN, INVITED_USER_1_USERNAME, INVITED_USER_2_EMAIL_ADDR,
            INVITED_USER_2_FIRST_NAME, INVITED_USER_2_LAST_NAME, INVITED_USER_2_USERNAME,
            INVITED_USER_3_EMAIL_ADDR, INVITED_USER_3_FIRST_NAME, INVITED_USER_3_LAST_NAME,
            ROUTE_GET, ROUTE_INVITE_GET, ROUTE_INVITE_SET, ROUTE_LOGOUT,
        }, workspaces::TARGETED_WORKSPACE,
    },
};

#[tokio::test]
async fn invite_new_users_by_form() {
    let client = async_test_client().await;

    let workspace = "ad5d4bf9-2e80-47b0-8454-1c431718b666";

    async_login(&client, DEFAULT_LOGIN).await;

    // Create a form with test data
    let invitation = InvitedMultipleUsersForm {
        users: vec![
            InvitedUserForm {
                first_name: INVITED_USER_1_FIRST_NAME,
                last_name: INVITED_USER_1_LAST_NAME,
                email: INVITED_USER_1_EMAIL_ADDR,
                phone: Some("0031699748558"),
                workspace_role: i16::from(WorkspaceRole::Contributor),
            },
            InvitedUserForm {
                first_name: INVITED_USER_1_FIRST_NAME,
                last_name: INVITED_USER_1_LAST_NAME,
                email: DUPLICATE_USER_1_EMAIL_ADDR,
                phone: None,
                workspace_role: i16::from(WorkspaceRole::Master),
            },
            InvitedUserForm {
                first_name: INVITED_USER_2_FIRST_NAME,
                last_name: INVITED_USER_2_LAST_NAME,
                email: INVITED_USER_2_EMAIL_ADDR,
                phone: Some("0683650773"),
                workspace_role: i16::from(WorkspaceRole::Stakeholder),
            },
            InvitedUserForm {
                first_name: INVITED_USER_2_FIRST_NAME,
                last_name: INVITED_USER_2_LAST_NAME,
                email: DUPLICATE_USER_2_EMAIL_ADDR,
                phone: None,
                workspace_role: i16::from(WorkspaceRole::Viewer),
            },
            InvitedUserForm {
                first_name: INVITED_USER_3_FIRST_NAME,
                last_name: INVITED_USER_3_LAST_NAME,
                email: INVITED_USER_3_EMAIL_ADDR,
                phone: None,
                workspace_role: i16::from(WorkspaceRole::Viewer),
            },
        ],
    };

    // Send submit request
    let response = client
        .post(format!("/workspaces/{workspace}/invite"))
        .body(invitation.body())
        .header(ContentType::Form)
        .dispatch()
        .await;

    // Clone the status before printing
    let status = response.status().clone();

    // Extract the ApiResponse containing the vector of strings with the tokens
    let invitation_response = response
        .into_json::<ApiResponse<Vec<String>>>()
        .await
        .unwrap();

    // Assert the submit request was successful
    assert_eq!(status, Status::Ok);

    // Get the redis cache
    let redis: &State<RedisMutex> = client
        .rocket()
        .state::<RedisMutex>()
        .expect("Redis state should be available")
        .into();

    // Loop through the tokens from the response
    for token in invitation_response.data.unwrap() {
        // Attempt getting the token from the cache
        assert!(cache::users::get_invite_token(redis, &token).await.is_ok());
    }
}

#[tokio::test]
async fn reinvite_user_by_id() {
    let client = async_test_client().await;

    let user_id = get_invited_user_id(&client, INVITED_USER_2_USERNAME).await;

    async_login(&client, ADMIN_LOGIN).await;

    let response = client
        .post(format!("/workspaces/{TARGETED_WORKSPACE}/re-invite/{user_id}"))
        .dispatch()
        .await;

    let status = response.status().clone();
    let deserialized_response = response.into_json::<ApiResponse<String>>().await;
    
    println!("{:?}", deserialized_response);
    assert_eq!(status, Status::Ok);

    let token = deserialized_response.unwrap().data.unwrap();

    // Get the redis cache
    let redis: &State<RedisMutex> = client
        .rocket()
        .state::<RedisMutex>()
        .expect("Redis state should be available")
        .into();

    assert!(cache::users::get_invite_token(redis, &token).await.is_ok());
}

#[tokio::test]
async fn set_password_after_receiving_invite() {
    let client = async_test_client().await;

    let (redis, token) = add_token_to_cache(&client, INVITED_USER_1_USERNAME.to_string()).await;

    // User clicks the link: The token should be recovered
    let response = client
        .get(format!("{ROUTE_INVITE_GET}{token}"))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    // User fills in the password: User will be activated
    let password = Password {
        first: DEFAULT_PASSWORD,
        second: DEFAULT_PASSWORD,
    };

    let response = client
        .put(format!("{ROUTE_INVITE_SET}{token}"))
        .body(password.body())
        .header(ContentType::Form)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    // Verify that the cache doesn't have the invitation token anymore
    assert!(cache::users::get_invite_token(&redis, &token)
        .await
        .is_err());

    // Login as newly created user
    async_login(&client, INVITED_USER_1_LOGIN).await;

    // Verify that the user status is upgraded
    let response = client
        .get(format!("{ROUTE_GET}{INVITED_USER_1_USERNAME}"))
        .dispatch()
        .await;

    // Copy the status for later assertion
    let status = response.status().clone();

    // Extract the data to print it to the screen
    let deserialized_response = response.into_json::<ApiResponse<PublicUser>>().await;

    assert_eq!(status, Status::Ok);
    assert!(deserialized_response.is_some());

    // Get the public user data from the deserialized response
    let public_user = deserialized_response.unwrap().data.unwrap();

    // Verify that the user is now active
    assert_eq!(public_user.status, i16::from(UserStatus::Active));
}

async fn add_token_to_cache(client: &Client, username: String) -> (&State<RedisMutex>, String) {
    // Get the redis cache
    let redis: &State<RedisMutex> = client
        .rocket()
        .state::<RedisMutex>()
        .expect("Redis state should be available")
        .into();

    // Generate a random token
    let token = cache::create_random_token(64);

    let user_id = get_invited_user_id(client, &username).await;

    // Convert the user ID to a UUID
    let user_id = Uuid::from_str(&user_id).unwrap();

    // Add the token to the cache
    assert!(cache::users::add_invite_token(&redis, &token, user_id)
        .await
        .is_ok());

    (redis, token)
}

async fn get_invited_user_id(client: &Client, username: &str) -> String {
    // Login as admin
    async_login(&client, ADMIN_LOGIN).await;

    // Send get request
    let response = client
        .get(format!("{ROUTE_GET}{username}"))
        .dispatch()
        .await;

    // Assert the get request was successful
    assert_eq!(response.status(), Status::Ok);

    // Extract the data
    let deserialized_response = response.into_json::<ApiResponse<PublicUser>>().await;

    // Get the public user data from the deserialized response
    let public_user = deserialized_response.unwrap().data.unwrap();

    // Verify that the user is invited
    assert_eq!(public_user.status, i16::from(UserStatus::Invited));

    // Log out
    let logout_response = client.post(ROUTE_LOGOUT).dispatch().await;

    // Assert that the logout request was handled succesfully
    assert_eq!(logout_response.status(), Status::Ok);

    public_user.id.to_string()
}
