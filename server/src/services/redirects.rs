use axum::{Router, extract::Path, response::Redirect, routing::get};
use tower_http::trace::{DefaultOnRequest, TraceLayer};

type RedirectPair = (&'static str, &'static str);
type RedirectList = &'static[RedirectPair];

//TODO: MOVE TO server.toml CONFIG FILE!! WAY NICER
const REDIRECTS: RedirectList = &[
    ("x-vs-wayland", "https://canartuc.medium.com/x11-vs-wayland-the-40-year-display-server-war-explained-37ac8bb0d720")
];

pub fn router() -> Router {
    Router::new()
        .route("/{*key}", get(handle_redirect_list))
        .route("/", get(Redirect::to("/")))
        .layer(TraceLayer::new_for_http()
            .on_request(DefaultOnRequest::new())
        )
}

async fn handle_redirect_list(Path(path): Path<String>) -> Redirect {
    Redirect::to(
        REDIRECTS.iter()
        .find(|(uri, _)| uri == &path)
        .map(|(_, link)| *link)
        .unwrap_or("/404")
    )
}
