use axum::{Router, body::Body, extract::{Path, State}, http::StatusCode, response::Json, routing::get};

use crate::cfg::ServerConfig;

pub fn router() -> Router<ServerConfig> {
    //      /blog/content
    Router::new()
        .route("/content/{*id}", get(post))
        .route("/assets/{*id}", get(asset))
        .route("/metadata", get(index))
        .route("/metadata/{*filter}", get(index_lookup))
}

async fn fetch_from_cache_server(context: &ServerConfig, route: &str, file: &str) -> Result<axum::response::Response, StatusCode> {
    let url = reqwest::Url::parse([&context.markdown_cache_server, route, file].join("/").as_str()).unwrap();

    let res = reqwest::get(url).await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    let res = axum::http::Response::from(res);
    let res =  res.map(|body| Body::new(body));

    Ok(res)
}

async fn post(State(context): State<ServerConfig>, Path(path): Path<String>) -> Result<axum::response::Response, StatusCode> {
    fetch_from_cache_server(&context, "post",&path).await
}

async fn asset(State(context): State<ServerConfig>, Path(path): Path<String>) -> Result<axum::response::Response, StatusCode> {
    fetch_from_cache_server(&context, "assets",&path).await
}

async fn index() -> Json<String> {
    todo!("serve post metadata for indexing")
}

async fn index_lookup(Path(path): Path<String>) -> Json<String> {
    todo!("serve metadata for posts by filter (called with: {})", path)
}
