pub mod blog;

use axum::{Router, extract::Path, response::Redirect, routing::get};
use tower_http::trace::{DefaultOnRequest, TraceLayer};

pub fn router() -> Router {
    Router::new()
        .nest_service("/blog", blog::router())
}
