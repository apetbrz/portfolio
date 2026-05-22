use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use reqwest::Url;
use tracing::{Level, event};

pub fn router() -> Router<Url> {
    //           /blog
    Router::new()
        .route("/content/{*id}", get(post))
        .route("/assets/{*id}", get(asset))
        .route("/metadata", get(index))
        .route("/metadata/{*filter}", get(post_meta))
}

async fn fetch_from_cache_server(
    blog_server: &Url,
    route: String
) -> Result<axum::response::Response, StatusCode> {
    let Ok(url) = blog_server.join(&route)
    else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    event!(Level::DEBUG, "fetching: {}", url);

    let res = reqwest::get(url)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    let res = axum::http::Response::from(res);
    let res = res.map(Body::new);

    Ok(res)
}

fn append_path(route: &str, path: &str) -> String {
    [route, path].join("/")
}

async fn post(
    State(blog_server): State<Url>,
    Path(path): Path<String>,
) -> Result<axum::response::Response, StatusCode> {
    fetch_from_cache_server(&blog_server, append_path("post", &path)).await
}

async fn asset(
    State(blog_server): State<Url>,
    Path(path): Path<String>,
) -> Result<axum::response::Response, StatusCode> {
    fetch_from_cache_server(&blog_server, append_path("assets", &path)).await
}

async fn index(
    State(blog_server): State<Url>,
) -> Result<axum::response::Response, StatusCode> {
    fetch_from_cache_server(&blog_server, "meta".into()).await
}

async fn post_meta(
    State(blog_server): State<Url>,
    Path(path): Path<String>,
) -> Result<axum::response::Response, StatusCode> {
    fetch_from_cache_server(&blog_server, append_path("meta", &path)).await
}
