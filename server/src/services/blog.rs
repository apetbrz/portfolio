use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tracing::{Level, event};

use crate::cfg::ServerConfig;

pub fn router() -> Router<ServerConfig> {
    //           /blog
    Router::new()
        .route("/content/{*id}", get(post))
        .route("/assets/{*id}", get(asset))
        .route("/metadata", get(index))
        .route("/metadata/{*filter}", get(post_meta))
}

async fn fetch_from_cache_server(
    context: &ServerConfig,
    route: String
) -> Result<axum::response::Response, StatusCode> {
    let Ok(url) = context.markdown_cache_server.join(&route)
    else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    event!(Level::DEBUG, "fetching: {}", url);

    let res = reqwest::get(url)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    let res = axum::http::Response::from(res);
    let res = res.map(|body| Body::new(body));

    Ok(res)
}

fn append_path(route: &str, path: &str) -> String {
    [route, path].join("/")
}

async fn post(
    State(context): State<ServerConfig>,
    Path(path): Path<String>,
) -> Result<axum::response::Response, StatusCode> {
    fetch_from_cache_server(&context, append_path("post", &path)).await
}

async fn asset(
    State(context): State<ServerConfig>,
    Path(path): Path<String>,
) -> Result<axum::response::Response, StatusCode> {
    fetch_from_cache_server(&context, append_path("assets", &path)).await
}

async fn index(
    State(context): State<ServerConfig>,
) -> Result<axum::response::Response, StatusCode> {
    fetch_from_cache_server(&context, "meta".into()).await
}

async fn post_meta(
    State(context): State<ServerConfig>,
    Path(path): Path<String>,
) -> Result<axum::response::Response, StatusCode> {
    fetch_from_cache_server(&context, append_path("meta", &path)).await
}
