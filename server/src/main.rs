use std::env;
use std::net::SocketAddr;

use axum::Router;
use config::Config;
use tower_http::services::{ServeDir, ServeFile};
use tracing::{Level, event};

//mod api;
mod services;
mod cfg;

use crate::cfg::ServerConfig;

const PORT: &str = env!("PORT");
const ASSETS: &str = env!("ASSETS");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    event!(Level::INFO, "starting...");

    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .expect("config building");

    let port = settings.get_int("PORT").expect("key PORT in config") as u16;
    let blog_server = match settings.get_string("BLOG_CACHE_SERVER") {
        Ok(str) => {
            event!(Level::INFO, "blog service set to {str}");
            Some(str)
        }
        Err(e) => {
            event!(Level::WARN, "could not load blog provider server url from config: {e:?}");
            event!(Level::WARN, "continuing without blog endpoints...");
            None
        }
    };
    let do_blog = blog_server.is_some();

    let context = ServerConfig::new(blog_server.unwrap_or("".into()));
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    event!(Level::INFO, "config initialized");

    let app = Router::new()
        // SvelteKit files
        .fallback_service(ServeFile::new(format!("{ASSETS}/index.html")))
        .nest_service("/_app", ServeDir::new(format!("{ASSETS}/_app/")))
        .nest_service("/assets", ServeDir::new(format!("{ASSETS}/assets/")))

        // favicon
        .route_service( "/favicon.svg", ServeFile::new(format!("{ASSETS}/favicon.svg")))

        // redirects
        .nest_service("/r", services::redirects::router().with_state(context.clone()))

        //config
        .with_state(context.clone());

    let app = if do_blog {
        app.nest_service("/blog", services::blog::router().with_state(context.clone()))
    }
    else { app };

    event!(Level::INFO, "routes initialized");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect(&format!("failed to bind to TCP socket at port {PORT}"));

    event!(Level::INFO, "...listening on {addr}!");

    axum::serve(listener, app).await.unwrap();
}
