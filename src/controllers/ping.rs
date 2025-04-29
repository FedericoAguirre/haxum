use axum::{Router, http::StatusCode, response::IntoResponse, routing::get, extract::State};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use super::super::services::redis_service::{ping}
// The easiest way to implement a handler is to use async functions that return a type that implements IntoResponse
// This can be a tuple of (StatusCode, a' string) or any other type that implements IntoResponse
async fn ping_root(State(pool): State<Pool<RedisConnectionManager>>) -> impl IntoResponse {
    let result = ping(&pool).await;
    match result {
        Ok(message) => (StatusCode::OK, message),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("Failed to ping database")),
    }
}

// The routes function is where we define the routes for this controller
// The routes function is called in the main.rs file to register the routes with the axum router
pub fn routes() -> Router {
    Router::new().route("/", get(ping_root))
}

#[cfg(test)]
mod tests {
    use super::super::super::services::redis_service::get_pool;
    use super::*;
    use axum::http::StatusCode;
    use axum::test_helpers::TestClient;

    #[tokio::test]
    async fn test_ping_root() {
        let pool = get_pool().await;
        let app: axum::routing::Router = axum::routing::Router::new()
            .route("/", get(ping_root))
            .with_state(pool);
        let response = TestClient::new(app).get("/").await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.text().await, "HELLO THERE!");
    }
}
