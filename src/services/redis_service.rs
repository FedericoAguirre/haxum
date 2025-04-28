use bb8::Pool;
use bb8_redis::RedisConnectionManager;

pub async fn get_pool() -> Pool<RedisConnectionManager> {
    tracing::info!("Creating Redis connection pool...");
    let manager = RedisConnectionManager::new("redis://127.0.0.1:6379").unwrap();
    let pool = bb8::Pool::builder().build(manager).await.unwrap();
    tracing::info!("Redis connection pool created");
    pool
}
