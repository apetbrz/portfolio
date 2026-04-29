use axum::{Router, body::{Body, Bytes}, extract::Path, handler::Handler, http::StatusCode, response::{Html, Json, Response}, routing::get};
use cached::proc_macro::{cached};
use http::Request;
use http_body_util::BodyExt;

const ASSETS: &str = env!("ASSETS");

struct Post { path: String, content: String }

pub fn router() -> Router {
    //      /api/blog
    Router::new()
        .route("/post/{*id}", get(post))
        .route("/posts", get(index))
        .route("/posts/{*filter}", get(index_lookup))
}

#[cached]
fn get_file(path: String) -> String {
    let contents = std::fs::read(path)
        .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
        .unwrap();

    let parser = pulldown_cmark::Parser::new(&contents);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    html
}

fn find_post_path_by_id(path: String) -> String {
    format!("{ASSETS}/blog/{path}/main.md")
}

async fn post(Path(path): Path<String>) -> Result<axum::response::Response, StatusCode> {
    let url = reqwest::Url::parse("http://localhost:3123/post/").unwrap().join(&path).unwrap();
    eprintln!("{}",url.path());

    let res = reqwest::get(url).await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    let mut response_builder = Response::builder().status(res.status());
    *response_builder.headers_mut().unwrap() = res.headers().clone();

    response_builder
        .body(Body::from_stream(res.bytes_stream()))
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)
}

async fn index() -> Json<String> {
    todo!("serve post metadata for indexing")
}

async fn index_lookup(Path(path): Path<String>) -> Json<String> {
    todo!("serve metadata for posts by filter")
}
