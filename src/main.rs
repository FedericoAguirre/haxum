use axum::Router;
use haxum::router::router::create_router;
use haxum::services::redis_service::get_pool;
use haxum::services::tracing as tracing_service;

#[tokio::main]
async fn main() {
    tracing_service::init().await;
    tracing::info!("Starting haxum server...");
    tracing::info!("Creating Redis connection pool...");
    let pool = get_pool().await;
    tracing::info!("Redis connection pool created");
    tracing::info!("Creating router...");
    let app: Router = create_router().await;
    tracing::info!("haxum server started on port 3000");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
