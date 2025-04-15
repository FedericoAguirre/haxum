use axum::Router;
use haxum::router::router::create_router;

#[tokio::main]
async fn main() {
    let app: Router = create_router().await;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
