use axum::Router;
use haxum::router::router::create_router;
use haxum::services::redis_service::get_pool;
use haxum::services::tracing as tracing_service;
use redis::AsyncCommands;

#[tokio::main]
async fn main() {
    tracing_service::init().await;
    tracing::info!("Starting haxum server...");
    tracing::info!("Creating Redis connection pool...");
    let pool = get_pool().await;

    // Redis connection test
    {
        // Ping Redis before starting
        tracing::info!("Creating Redis connection...");
        let conn_result = pool.get().await;

        if conn_result.is_err() {
            tracing::error!("Failed to get Redis connection: {:?}", conn_result.err());
            return;
        }

        let mut conn = conn_result.unwrap();
        tracing::info!("Pinging Redis...");
        let result: String = conn.ping().await.unwrap();
        assert_eq!(result, "PONG");
    }

    tracing::info!("Redis is READY");
    tracing::info!("Creating router...");
    let app: Router = create_router().await;
    tracing::info!("haxum server started on port 3000");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
