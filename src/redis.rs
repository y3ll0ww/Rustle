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
//! redis-server ==deamonize yes
//! ```
//! > To verify: `redis-cli ping`
use redis::Client;
use rocket::tokio::sync::Mutex;
use std::sync::Arc;

pub type RedisMutex = Arc<Mutex<RedisPool>>;

// QWERTY Use env var in production
const REDIS_URL: &str = "redis://127.0.0.1:6379";

pub struct RedisPool {
    client: Client,
}

impl RedisPool {
    pub fn new(url: &str) -> redis::RedisResult<Self> {
        let client = Client::open(url)?;
        Ok(RedisPool { client })
    }

    pub async fn get_connection(&self) -> redis::RedisResult<redis::aio::Connection> {
        self.client.get_async_connection().await
    }
}

pub fn redis_fairing() -> impl rocket::fairing::Fairing {
    rocket::fairing::AdHoc::on_ignite("Redis", |rocket| async {
        match RedisPool::new(REDIS_URL) {
            Ok(pool) => rocket.manage(Arc::new(Mutex::new(pool))),
            Err(e) => {
                // QWERTY perhaps panic here.
                eprintln!("Failed to initialize Redis: {}", e);
                rocket
            }
        }
    })
}
