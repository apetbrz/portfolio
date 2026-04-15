use axum::{Router, http::Uri, response::Html, routing::get};
use std::env;
use tower_http::{services::{ServeDir, ServeFile}, trace::{DefaultOnRequest, TraceLayer}};

mod redirects;

const DIST_PATH: &str = "../dist";

#[tokio::main]
async fn main() {
    println!("starting!");

    let port = if env::var("IS_RELEASE").is_ok() {
        4000
    } else {
        3000
    };

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .nest_service("/_app", ServeDir::new(format!("{DIST_PATH}/_app/")))
        .nest_service("/assets", ServeDir::new(format!("{DIST_PATH}/assets/")))
        .route_service(
            "/favicon.svg",
            ServeFile::new(format!("{DIST_PATH}/favicon.svg")),
        )
        .nest_service("/r", redirects::router())
        .fallback_service(get(async |p: Uri| {
            Html(
                tokio::fs::read_to_string(format!("{DIST_PATH}/index.html"))
                    .await
                    .expect(&format!["missing index file at {}",p]),
            )
        }))
        .layer(TraceLayer::new_for_http()
            .on_request(DefaultOnRequest::new())
        );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    println!("listening on port {port}");

    axum::serve(listener, app).await.unwrap();
}
