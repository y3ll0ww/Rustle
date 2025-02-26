use redis::aio::Connection;
use rocket::local::{asynchronous::Client as AsyncClient, blocking::Client};

use crate::redis::RedisMutex;

pub mod users;

pub fn test_client() -> Client {
    Client::tracked(crate::rocket()).expect("valid rocket instance")
}

pub async fn async_test_client() -> AsyncClient {
    AsyncClient::tracked(crate::rocket())
        .await
        .expect("Valid Rocket instance")
}

pub async fn get_cache(client: &AsyncClient) -> Result<Connection, String> {
    let rocket = client.rocket();
    let redis_pool = rocket
        .state::<RedisMutex>()
        .expect("RedisPool should be managed");

    // Check token in Redis
    redis_pool
        .lock()
        .await
        .get_connection()
        .await
        .map_err(|e| e.to_string())
}
