use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use jsonwebtoken::{decode, DecodingKey, Validation};
use redis::AsyncCommands;
use rocket::{
    http::{ContentType, Header, Status},
    tokio,
};
use serde_json::json;

use crate::{
    api::ApiResponse,
    auth::{Claims, UserInfo, USER_KEY},
    forms::users::{InsertedUser, LoginForm, NewUserForm, Password},
    models::users::UserRole,
    tests::{async_test_client, get_cache, test_client},
};

const USERNAME: &str = "test_user";
const PASSWORD: &str = "strong_password";

#[test]
fn create_new_dummy_user() {
    let client = test_client();

    // Use valid date values for created_at and updated_at fields
    let created_at = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    );
    let updated_at = created_at;

    // Construct a JSON payload matching the User structure
    let payload = json!({
        "id": "a99b50c6-02e9-4142-95fe-35c3ccd4f147",  // Will be overwritten by `create_user` function with UUID
        "privilege": 3,
        "username": "y3ll0ww",
        "display_name": null,
        "email": "some@abc.nl",
        "password_hash": "password",
        "bio": null,
        "avatar_url": null,
        "created_at": created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),  // Convert to ISO 8601 format
        "updated_at": updated_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
    });

    // Send POST request to the correct endpoint `/users`
    let response = client
        .post("/user/create")
        .header(ContentType::JSON)
        .body(payload.to_string())
        .dispatch();

    // Assert that the response status is 200 (indicating success)
    assert_eq!(response.status(), Status::Ok);

    // Optionally, check the response body for success message
    let response_body = response.into_string().unwrap();
    println!("{response_body}");
    assert!(response_body.contains("User y3ll0ww created"));
}

/// Creates a new user in the database using form data.
/// 
/// ## Prerequisites
/// 
/// Make sure there is no standard user already added to the database or the test will fail.
#[test]
fn test_submit() {
    let client = test_client();

    // Create a form with test data
    let new_user = NewUserForm {
        username: USERNAME,
        password: Password {
            first: PASSWORD,
            second: PASSWORD,
        },
        email: "test@example.com",
    };

    // Send submit request
    let response = client
        .post("/user/form")
        .body(new_user.body()) // Use the formatted string as the body
        .header(ContentType::Form)
        .dispatch();

    // Assert the submit request was successful
    assert_eq!(response.status(), Status::Ok);

    // Extract the string from the response
    let response_str = &response.into_string().unwrap();

    // Assert that the response has InsertedUser with correct data
    match serde_json::from_str::<ApiResponse<InsertedUser>>(response_str) {
        Ok(api_response) => assert_eq!(api_response.data.unwrap().username, new_user.username),
        Err(e) => assert!(false, "{e}"),
    }
}

/// Logging in and logging out a user.
///
/// This test will log in the standard user and verify if the token and user information is
/// correctly added to the cache. Then it will log out the standard user and verify that said
/// information is removed from the cache.
///
/// ## Prerequisites
///
/// There should already be a standard user added to the database. This can be done by running the
/// test `test_submit`.
#[tokio::test]
async fn new_login_logout_user() {
    let client = async_test_client().await;
    let mut cache = get_cache(&client).await.unwrap();

    // Create a form with test data
    let login = LoginForm {
        username: USERNAME,
        password: PASSWORD,
    };

    // ==============================================================
    //  Logging in user
    // ==============================================================
    let login_response = client
        .post("/user/login")
        .header(ContentType::Form)
        .body(login.body())
        .dispatch()
        .await;

    // Assert the login request was successful
    assert_eq!(login_response.status(), Status::Ok);

    // Extract the generated JWT token from the login response
    let token: String = login_response
        .into_json::<ApiResponse<String>>()
        .await
        .unwrap()
        .data
        .unwrap();

    // Assert that the token is added as a key to the cache
    let token_value: String = match cache.get(&token).await {
        Ok(user_id) => user_id,
        Err(_) => return assert!(false, "Token should exist in Redis after login"),
    };

    // Get user ID from token (decode JWT)
    let secret = "your-secret-key".as_bytes(); // QWERTY Match your secret
    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .unwrap()
    .claims;

    // Asset that the token value and the user ID as the same
    assert_eq!(token_value, claims.sub, "Token value should be the user ID");

    // Define the key in the Redis pool for retreiving user data; contains the user ID
    let user_key = format!("{USER_KEY}{}", claims.sub);

    // Get and deserialize the UserInfo from the cache
    let user_info: UserInfo = cache
        .get(&user_key)
        .await
        .map_err(|_| assert!(false, "User info should exist in Redis after login"))
        .and_then(|user_value: String| {
            serde_json::from_str::<UserInfo>(&user_value)
                .map_err(|_| assert!(false, "Couldn't deserialize UserInfo"))
        })
        .unwrap();

    // Assert that the UserInfo contains the right data
    assert_eq!(user_info.username, USERNAME);
    assert_eq!(user_info.id, token_value);
    assert_eq!(user_info.role, UserRole::Reviewer);

    // ==============================================================
    //  Logging out user
    // ==============================================================
    let logout_response = client
        .post("/user/logout")
        .header(Header::new("Authorization", format!("Bearer {}", token)))
        .dispatch()
        .await;

    // Assert that the logout request was handled succesfully
    assert_eq!(logout_response.status(), Status::Ok);

    // Try to get the same keys from the cache after logout
    let token_exists: bool = cache.exists(&token).await.unwrap();
    let user_info_exists: bool = cache.exists(&user_key).await.unwrap();

    // Assert there is no token key after logout
    assert!(
        !token_exists,
        "Token should be removed from Redis after logout"
    );

    // Assert there is no UserInfo after logout
    assert!(
        !user_info_exists,
        "User info should be removed from Redis after logout"
    );
}

/// Removes a user with a given user ID from the database.
///
/// ## Prerequisites
///
/// The user should already exist in the database. This can be done by running the test
/// `test_submit`.
///
/// The tester should know the user ID from the desired user, then replace the `user_id` variable
/// inside the test accordingly.
#[test]
fn delete_user() {
    let client = test_client();

    // User ID: Change depending on which user tester wants to delete
    let user_id = "ddea1aca-34e2-4176-9126-067852f8440a";

    // Send delete request
    let response = client.delete(format!("/user/delete/{user_id}")).dispatch();

    // Assert the delete request was successful
    assert_eq!(response.status(), Status::Ok);
}