use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;

pub async fn get_pool() -> Pool<RedisConnectionManager> {
    tracing::info!("Creating Redis connection pool...");
    let manager = RedisConnectionManager::new("redis://127.0.0.1:6379").unwrap();
    let pool = bb8::Pool::builder().build(manager).await.unwrap();
    tracing::info!("Redis connection pool created");
    pool
}

pub async fn ping(pool: &Pool<RedisConnectionManager>) -> Result<String, String> {
    let mut conn = pool.get().await.map_err(|e| e.to_string())?;
    let result: String = conn.ping().await.map_err(|e| e.to_string())?;
    if result == "PONG" {
        Ok("HELLO THERE!".to_string())
    } else {
        Err("Failed to ping Redis".to_string())
    }
}

pub async fn test_redis_connection(pool: &Pool<RedisConnectionManager>) {
    // Ping Redis before starting
    tracing::info!("Creating Redis connection...");
    let conn_result = pool.get().await;

    if conn_result.is_err() {
        tracing::error!("Failed to get Redis connection: {:?}", conn_result.err());
        panic!("Redis is unavailable");
    }

    let mut conn = conn_result.unwrap();
    tracing::info!("Pinging Redis...");
    let result: String = conn.ping().await.unwrap();
    assert_eq!(result, "PONG");
}
