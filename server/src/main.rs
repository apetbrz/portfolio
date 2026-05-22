use std::net::SocketAddr;

use axum::Router;
use tower_http::{LatencyUnit, services::{ServeDir, ServeFile}, trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer}};
use tracing::{event, Level};

use crate::cfg::init_cfg;

mod cfg;
mod services;

#[tokio::main]
async fn main() {
    let level = match std::env::var_os("DEBUG_TRACE").is_some() {
        true => Level::DEBUG,
        false => Level::INFO
    };
    tracing_subscriber::fmt().with_max_level(level).init();

    event!(Level::INFO, "starting...");
    let cfg = init_cfg().expect("failed to init config");
    event!(Level::INFO, "config initialized");
    let app = Router::new()
        .route_service("/", ServeFile::new(cfg.assets.join("index.html")))
        .nest_service("/r", services::redirects::router(cfg.redirects))
        .nest_service("/blog", services::blog::router(cfg.blog_server))
        .layer(TraceLayer::new_for_http()
            .make_span_with(
                DefaultMakeSpan::new().level(Level::INFO)
            )
            .on_request(
                DefaultOnRequest::new().level(Level::INFO)
            )
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(LatencyUnit::Micros)
            )
        )
        .nest_service(
            "/_app",
            ServeDir::new(cfg.assets.join("_app/")),
        )
        .nest_service(
            "/assets",
            ServeDir::new(cfg.assets.join("assets/")),
        )
        .route_service(
            "/favicon.svg",
            ServeFile::new(cfg.assets.join("favicon.svg")),
        )
        .fallback_service(ServeFile::new(cfg.assets.join("index.html")))
        .layer(TraceLayer::new_for_http().on_request(DefaultOnRequest::new().level(Level::DEBUG)))
    ;

    event!(Level::INFO, "routes initialized");

    #[cfg(debug_assertions)]
    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.dev_port.unwrap_or(cfg.port)));
    #[cfg(not(debug_assertions))]
    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("failed to bind to TCP socket at {addr}"));

    event!(Level::INFO, "...listening on {addr}!");

    axum::serve(listener, app).await.unwrap();
}
