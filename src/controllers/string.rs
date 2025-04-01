// Import the necessary models
use crate::models::string_body::StringBody;

// Import the necessary axum API modules
use axum::{Json, Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};

// Used to seriealize in line
use serde_json::json;

// The easiest way to implement a handler is to use async functions that return a type that implements IntoResponse
// This can be a tuple of (StatusCode, Json<T>) or any other type that implements IntoResponse
// In this case we are returning a tuple of (StatusCode, Json<T>)
async fn get_string(Path(key): Path<String>) -> impl IntoResponse {
    match key.as_str() {
        "hello" => {
            return (
                StatusCode::OK,
                Json(StringBody::new("hello".to_string(), "world".to_string())),
            )
                .into_response();
        }
        "damn" => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "SERVER IS DOWN"})),
            )
                .into_response();
        }
        _ => {
            return (StatusCode::NOT_FOUND, Json(json!({"error": "NOT FOUND"}))).into_response();
        }
    }
}

// The routes function is where we define the routes for this controller
// The routes function is called in the main.rs file to register the routes with the axum router
pub fn routes() -> Router {
    Router::new().route("/get_string/{key}", get(move |key| get_string(key)))
}
