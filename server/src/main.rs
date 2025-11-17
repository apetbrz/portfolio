use axum::{response::Html, routing::get, Router};
use std::env;
use tower_http::services::{ServeDir, ServeFile};

const DIST_PATH: &str = "../dist";

#[tokio::main]
async fn main() {
    println!("starting!");

    let port = if env::var("IS_RELEASE").is_ok() {
        4000
    } else {
        3000
    };

    let app = Router::new()
        .nest_service("/_app", ServeDir::new(format!("{DIST_PATH}/_app/")))
        .nest_service("/assets", ServeDir::new(format!("{DIST_PATH}/assets/")))
        .route_service(
            "/favicon.svg",
            ServeFile::new(format!("{DIST_PATH}/favicon.svg")),
        )
        .fallback_service(get(async || {
            Html(
                tokio::fs::read_to_string(format!("{DIST_PATH}/index.html"))
                    .await
                    .expect("missing index file! "),
            )
        }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    println!("starting on port {port}");

    axum::serve(listener, app).await.unwrap();
}
