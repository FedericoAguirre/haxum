use axum::Router;
use haxum::router::router::create_router;
use haxum::services::tracing as tracing_service;

#[tokio::main]
async fn main() {
    tracing_service::init().await;
    tracing::info!("Starting haxum server...");

    let app: Router = create_router().await;
    tracing::info!("haxum server started on port 3000");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
