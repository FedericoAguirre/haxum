use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

// The easiest way to implement a handler is to use async functions that return a type that implements IntoResponse
// This can be a tuple of (StatusCode, a' string) or any other type that implements IntoResponse
async fn ping_root() -> impl IntoResponse {
    (StatusCode::OK, "PONG")
}

// The routes function is where we define the routes for this controller
// The routes function is called in the main.rs file to register the routes with the axum router
pub fn routes() -> Router {
    Router::new().route("/", get(ping_root))
}
