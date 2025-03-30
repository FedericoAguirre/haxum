use axum::Router;
// We import the controllerrs here so we can use them in the main function
use haxum::controllers::{hello as hello_controller, ping as ping_controller};

#[tokio::main]
async fn main() {
    // We build our application with multiple routers using Router::merge
    let app = Router::new()
        .merge(hello_controller::routes())
        .merge(ping_controller::routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
