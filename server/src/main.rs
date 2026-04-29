use std::env;

use axum::{http::Uri, response::Html, routing::get, Router};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::{DefaultOnRequest, TraceLayer},
};

mod api;
mod services;

const PORT: &str = env!("PORT");
const ASSETS: &str = env!("ASSETS");

#[tokio::main]
async fn main() {
    println!("starting...");

    tracing_subscriber::fmt::init();

    let app = Router::new()
        // SvelteKit files
        .nest_service("/_app", ServeDir::new(format!("{ASSETS}/_app/")))
        .nest_service("/assets", ServeDir::new(format!("{ASSETS}/assets/")))

        // Redirects
        .nest_service("/r", services::redirects::router())

        // API paths
        .nest_service("/api", api::router())

        // favicon
        .route_service(
            "/favicon.svg",
            ServeFile::new(format!("{ASSETS}/favicon.svg")),
        )

        // all else -> send index (SvelteKit handles 404s)
        .fallback_service(
            get(async |p: Uri| {
                Html(
                    tokio::fs::read_to_string(format!("{ASSETS}/index.html"))
                        .await
                        .expect(&format!["missing index file at {}", p]),
                )
            })
            .layer(TraceLayer::new_for_http().on_request(DefaultOnRequest::new())),
        );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}"))
        .await
        .expect(&format!("failed to bind to TCP socket at port {PORT}"));

    println!("listening on port {PORT}");

    axum::serve(listener, app).await.unwrap();
}
