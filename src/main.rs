use axum::Router;
use haxum::router::router::create_router;
use tracing_subscriber::fmt::{self};

#[tokio::main]
async fn main() {
    // Configure a custom event formatter
    let format = fmt::format()
        .with_level(true) // don't include levels in formatted output
        .with_target(true) // don't include targets
        .with_thread_ids(true) // include the thread ID of the current thread
        .with_thread_names(true) // include the name of the current thread
        .with_timer(fmt::time::ChronoLocal::rfc_3339()); // use RFC 3339 format for timestamps

    // Create a `fmt` subscriber that uses our custom event format, and set it
    // as the default.
    tracing_subscriber::fmt().event_format(format).init();
    tracing::info!("Starting haxum server...");

    let app: Router = create_router().await;
    tracing::info!("haxum server started on port 3000");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
