use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

// This is a function  that returns a text response with a status code of 200
async fn ping_root() -> impl IntoResponse {
    (StatusCode::OK, "PONG")
}

pub fn routes() -> Router {
    Router::new().route("/", get(ping_root))
}
