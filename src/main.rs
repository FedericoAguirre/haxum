use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};

// This is a function  that returns a text response with a status code of 200
async fn ping_root() -> impl IntoResponse {
    (StatusCode::OK, "PONG")
}

// Add to Cargo.toml file serde and serde-json to use Serialize and Deserialize
#[derive(Deserialize, Serialize)]
pub struct StringBody {
    key: String,
    value: String,
}

// The easiest way to implment a handler is to use async functions that return a type that implements IntoResponse
// This can be a tuple of (StatusCode, Json<T>) or any other type that implements IntoResponse
// In this case we are returning a tuple of (StatusCode, Json<T>)
async fn hello_world() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(StringBody {
            key: "Hello".to_string(),
            value: "World".to_string(),
        }),
    )
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(ping_root))
        .route("/hello", get(hello_world));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
