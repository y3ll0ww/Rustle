//! This file enables the ability to have Redis caching functionality.
//!
//! For Redis to work, it should first be running its pool on a dedicated port.
//!
//! ### 1. Install Redis
//! ```bash
//! apt update
//! apt install -y redis-server
//! ```
//!
//! ### 2. Run Redis server
//! ```bash
//! redis-server --daemonize yes
//! ```
//! > To verify: `redis-cli ping`
use rand::{distributions::Alphanumeric, Rng};
use redis::{aio::Connection, AsyncCommands, Client, RedisResult};
use rocket::{fairing::Fairing, tokio::sync::Mutex};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

use crate::api::{ApiResponse, Error, Null};

pub mod users;
pub mod workspaces;

pub type RedisMutex = Arc<Mutex<RedisPool>>;

// QWERTY Use env var in production
const REDIS_URL: &str = "redis://127.0.0.1:6379";

pub struct RedisPool {
    client: Client,
}

impl RedisPool {
    pub fn new(url: &str) -> RedisResult<Self> {
        let client = Client::open(url)?;
        Ok(RedisPool { client })
    }

    pub async fn get_connection(&self) -> RedisResult<Connection> {
        self.client.get_async_connection().await
    }

    // Method to get data from Redis cache
    pub async fn get_from_cache<T>(&self, key: &str) -> Result<Option<T>, Error<Null>>
    where
        T: DeserializeOwned,
    {
        let mut con = self.get_connection().await.map_err(|e| {
            ApiResponse::internal_server_error(format!("Cache connection error: {e}"))
        })?;

        // Try to get the cached data
        let cached_data: Option<String> = con.get(key).await.map_err(|e| {
            ApiResponse::internal_server_error(format!("Cache retrieval error: {e}"))
        })?;

        // If data exists, deserialize it
        match cached_data {
            Some(data) => {
                let deserialized_data: T = serde_json::from_str(&data).map_err(|e| {
                    ApiResponse::conflict(format!("Failed to deserialize: {}", e), data)
                })?;
                Ok(Some(deserialized_data))
            }
            None => Ok(None),
        }
    }

    // Method to set data to Redis cache with optional TTL
    pub async fn set_to_cache<T>(
        &self,
        key: &str,
        value: &T,
        ttl: Option<u64>,
    ) -> Result<(), Error<Null>>
    where
        T: Serialize,
    {
        let mut con = self.get_connection().await.map_err(|e| {
            ApiResponse::internal_server_error(format!("Couldn't optain Redis connection: {e}"))
        })?;

        let serialized = serde_json::to_string(value)
            .map_err(|e| ApiResponse::internal_server_error(format!("Failed to serialize: {e}")))?;

        // Set the data to Redis with optional TTL
        if let Some(ttl_value) = ttl {
            let ttl_usize: usize = ttl_value.try_into().map_err(|e| {
                ApiResponse::internal_server_error(format!("TTL conversion error: {e}"))
            })?;
            let _: () = con
                .set_ex(key, serialized, ttl_usize)
                .await
                .map_err(|e| ApiResponse::internal_server_error(format!("Redis SET error: {e}")))?;
        } else {
            let _: () = con
                .set(key, serialized)
                .await
                .map_err(|e| ApiResponse::internal_server_error(format!("Redis SET error: {e}")))?;
        }

        Ok(())
    }

    // Method to remove data to Redis cache
    pub async fn remove_from_cache(&self, key: &str) -> Result<(), Error<String>> {
        let mut conn = self.get_connection().await.map_err(|e| {
            ApiResponse::internal_server_error(format!("Couldn't optain Redis connection: {e}"))
        })?;

        let _: () = conn
            .del(key)
            .await
            .map_err(|e| ApiResponse::internal_server_error(format!("Redis DEL error: {e}")))?;

        Ok(())
    }
}

pub fn redis_fairing() -> impl Fairing {
    rocket::fairing::AdHoc::on_ignite("Redis", |rocket| async {
        match RedisPool::new(REDIS_URL) {
            Ok(pool) => rocket.manage(Arc::new(Mutex::new(pool))),
            Err(e) => {
                // QWERTY perhaps panic here instead of returning limited rocket.
                eprintln!("Failed to initialize Redis: {}", e);
                rocket
            }
        }
    })
}

pub fn create_random_token(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect::<String>()
}
