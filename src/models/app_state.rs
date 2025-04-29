use bb8::Pool;
use bb8_redis::RedisConnectionManager;

#[derive(Debug, Clone)]
pub struct AppState {
    pub redis_pool: Pool<RedisConnectionManager>,
}
impl AppState {
    pub fn new(redis_pool: Pool<RedisConnectionManager>) -> Self {
        AppState { redis_pool }
    }
}
