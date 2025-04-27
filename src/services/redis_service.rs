use bb8::{Pool, PooledConnection};
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;

async fn get_pool() -> Pool<RedisConnectionManager> {
    tracing::info!("Creating Redis connection pool...");
    let manager = RedisConnectionManager::new("redis://localhost:6379").unwrap();
    let pool = bb8::Pool::builder().build(manager).await.unwrap();
    tracing::info!("Redis connection pool created");
    pool
}
