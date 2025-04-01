// Import the necessary models
use crate::models::string_body::StringBody;

// Import the necessary axum API modules
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};

// The easiest way to implement a handler is to use async functions that return a type that implements IntoResponse
// This can be a tuple of (StatusCode, Json<T>) or any other type that implements IntoResponse
// In this case we are returning a tuple of (StatusCode, Json<T>)
async fn hello_world() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(StringBody::new("hello".to_string(), "world".to_string())),
    )
}

// The routes function is where we define the routes for this controller
// The routes function is called in the main.rs file to register the routes with the axum router
pub fn routes() -> Router {
    Router::new().route("/hello", get(hello_world))
}
