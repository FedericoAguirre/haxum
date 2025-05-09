use std::sync::Arc;

use crate::models::app_state::AppState;
use axum::Router;
use axum::extract::State;
// We import the controllers here so we can use them in the create_router function
use crate::controllers::{
    hello as hello_controller, ping as ping_controller, string as string_controller,
};

pub async fn create_router(State(state): State<Arc<AppState>>) -> Router {
    // Create a new router and merge all the controllers into it
    Router::new()
        .merge(hello_controller::routes())
        .merge(ping_controller::routes((State, state)))
        .merge(string_controller::routes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, extract::Request, http::StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_create_router() {
        let app_state = AppState {
            redis_pool: crate::services::redis_service::get_pool().await,
        };
        let state = State(app_state.clone());
        let app = create_router(state.clone()).await;

        // Test the hello_controller route
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Test the ping_controller route
        let app = create_router(state.clone()).await;

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let app = create_router(state.clone()).await;
        // Test the string_controller route
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/get_string/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
// This module contains the router for the application. It imports all the controllers and merges them into a single route
