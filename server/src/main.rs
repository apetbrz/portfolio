use axum::{response::Html, routing::get, Router};
use std::env;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let port = if env::var("IS_RELEASE").is_ok() {
        4000
    } else {
        3000
    };

    let app = Router::new()
        .route_service(
            "/",
            get(async || {
                Html(
                    tokio::fs::read_to_string("../client/build/index.html")
                        .await
                        .expect("missing index file!"),
                )
            }),
        )
        .fallback_service(ServeDir::new("../client/build"));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("starting on port {}", port);

    axum::serve(listener, app).await.unwrap();
}
