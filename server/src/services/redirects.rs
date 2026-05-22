use std::collections::HashMap;

use axum::{Router, extract::{Path, State}, response::Redirect, routing::get};
use tracing::{Level, event};

pub fn router(state: Option<HashMap<String, String>>) -> Router {
    match state {
        Some(redirects) => {
            event!(Level::INFO, "redirects service on!");
            base_router()
                .merge(redirects_handler(redirects))
        }
        None => {
            event!(Level::INFO, "redirects service settings missing, ignoring...");
            base_router()
        }
    }
}

fn base_router() -> Router {
    Router::new()
        .route("/", get(Redirect::to("/")))
}
fn redirects_handler(state: HashMap<String, String>) -> Router {
    Router::new()
        .route("/{*key}", get(handle_redirect_list))
        .with_state(state)
}

async fn handle_redirect_list(State(redirects): State<HashMap<String, String>>, Path(path): Path<String>) -> Redirect {
    Redirect::to(
        redirects
            .get(&path)
            .map(|s| s.as_str())
            .unwrap_or("/")
    )
}
