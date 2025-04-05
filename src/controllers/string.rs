// Import the necessary models
use crate::models::string_body::StringBody;

// Import the necessary axum API modules
use axum::{
    Router,
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

// Used to seriealize in line
use serde_json::json;

// Use regex to validate the request body
use regex::Regex;

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

async fn set_string(Json(string_body): Json<StringBody>) -> impl IntoResponse {
    // Here you would typically save the string to a database or perform some action
    // For this example, we will just return the string back

    // TODO: Manage HTTP/1.1 415 Unsupported Media Type error
    // TODO: Manage HTTP/1.1 422 Unprocessable Entity error

    let key_pattern = Regex::new(r"^[a-zA-Z0-9:\-_]+$").unwrap();
    let value_pattern = Regex::new(r"^.+$").unwrap();

    if !key_pattern.is_match(&string_body.key) || !value_pattern.is_match(&string_body.value) {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "Key or value is in invalid format"})),
        )
            .into_response();
    }

    (StatusCode::OK, Json(string_body)).into_response()
}

// The routes function is where we define the routes for this controller
// The routes function is called in the main.rs file to register the routes with the axum router
pub fn routes() -> Router {
    Router::new()
        .route("/get_string/{key}", get(move |key| get_string(key)))
        .route("/set_string", post(set_string))
}
