use std::net::SocketAddr;

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};
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

    #[cfg(debug_assertions)]
    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.dev_port.unwrap_or(cfg.port)));

    #[cfg(not(debug_assertions))]
    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port));

    event!(Level::INFO, "config initialized");

    let app = Router::new();

    // SvelteKit files
    let app = app
        .fallback_service(ServeFile::new(cfg.assets.join("index.html")))
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
        );

    // redirects
    let app = if let Some(redirects) = cfg.redirects {
        event!(Level::INFO, "redirects service on!");
        app.nest_service("/r", services::redirects::router().with_state(redirects))
    } else {
        app
    };

    // blog
    let app = if let Some(blog_server) = cfg.blog_server {
        event!(Level::INFO, "blog service on!");
        app.nest_service(
            "/blog",
            services::blog::router().with_state(reqwest::Url::parse(&blog_server).unwrap()),
        )
    } else {
        app
    };

    event!(Level::INFO, "routes initialized");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect(&format!("failed to bind to TCP socket at {addr}"));

    event!(Level::INFO, "...listening on {addr}!");

    axum::serve(listener, app).await.unwrap();
}
