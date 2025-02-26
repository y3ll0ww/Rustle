use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

use jsonwebtoken::{decode, DecodingKey, Validation};
use redis::{aio::Connection, AsyncCommands};
use rocket::{
    http::{ContentType, Header, Status},
    tokio,
};
use serde_json::json;

use crate::{
    api::ApiResponse,
    auth::{Claims, UserInfo, USER_KEY},
    forms::users::{LoginForm, NewUserForm, Password},
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
/// It will submit a [`NewUserForm`], check the response status code and extract the returned
/// `token` from the response.
/// 
/// Then it will validate the cache to check if the newly created user is logged in.
/// 
/// ## Prerequisites
/// 
/// Make sure there is no standard user already added to the database or the test will fail.
#[tokio::test]
async fn submit_new_user_by_form() {
    let client = async_test_client().await;
    let mut cache = get_cache(&client).await.unwrap();

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
        .dispatch()
        .await;

    // Assert the submit request was successful
    assert_eq!(response.status(), Status::Ok);

    // Extract the generated JWT token from the submit response
    let token: String = response
        .into_json::<ApiResponse<String>>()
        .await
        .unwrap()
        .data
        .unwrap();

    // Validate that the information is added to the cache; user is logged in
    let _ = validate_cache_after_login(&token, &mut cache);
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
async fn login_existing_user_then_logout() {
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

    // Validate that the information is added to the cache; user is logged in
    let user_id = validate_cache_after_login(&token, &mut cache).await;

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
    let user_info_exists: bool = cache.exists(&cache_user_key(&user_id)).await.unwrap();

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
fn delete_existing_user_by_id() {
    let client = test_client();

    // User ID: Change depending on which user tester wants to delete
    let user_id = "fe0b96a1-d86f-4f00-93b1-d708b777cb88";

    // Send delete request
    let response = client.delete(format!("/user/delete/{user_id}")).dispatch();

    // Assert the delete request was successful
    assert_eq!(response.status(), Status::Ok);
}

/// This helper function goes through some repeated steps for validating the cache after a user is
/// logged in.
/// 
/// It does the following to assure the cache is correct:
/// - Extracts the provided `token` as key from the cache.
/// - Decodes `Claims` using the secret key
/// - Compares the user ID's from `Claims` and the value from the `token` key
/// - Extracts the `UserInfo` from the users key (containing the user ID)
/// - Verifies the data inside `UserInfo`
/// 
/// ### Parameters
/// 
/// * `token: String` - The token that's returned as a result from a request.
/// * `cache: &mut Connection` - A mutable reference to the cache (Redis) instance.
/// 
/// ### Returns
/// 
/// * `String` - The result of this function, if it executes successfully, is a `String` containing
///   the **user ID**, since any user ID references are out of scope at the end of this function.
async fn validate_cache_after_login(token: &String, cache: &mut Connection) -> String {
    // Assert that the token is added as a key to the cache
    let token_value: String = match cache.get(&token).await {
        Ok(user_id) => user_id,
        Err(_) => panic!("Token should exist in Redis after login"),
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

    // Assert that the token value and the user ID as the same
    assert_eq!(token_value, claims.sub, "Token value should be the user ID");

    // Get and deserialize the UserInfo from the cache
    let user_info: UserInfo = cache
        .get(&cache_user_key(&claims.sub))
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

    user_info.id
}

/// Define the key in the Redis pool for retreiving user data; contains the user ID
fn cache_user_key(user_id: &String) -> String {
    format!("{USER_KEY}{user_id}")
}