use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(handler_hello));

    // region:         ---Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> Listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    // endregion:      ---Start Server
}

// region:         ---Handler Hello
async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");
    Html("Hello, <strong>world!!!</strong>")
}
// endregion:      ---Handler Hello
