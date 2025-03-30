use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use haxum::controllers::ping as ping_controller;
use haxum::models::string_body::StringBody;

// The easiest way to implement a handler is to use async functions that return a type that implements IntoResponse
// This can be a tuple of (StatusCode, Json<T>) or any other type that implements IntoResponse
// In this case we are returning a tuple of (StatusCode, Json<T>)
async fn hello_world() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(StringBody::new("hello".to_string(), "world".to_string())),
    )
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/hello", get(hello_world))
        .merge(ping_controller::routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
