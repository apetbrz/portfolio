use axum::{Router, extract::Path, response::Redirect, routing::get};

type RedirectPair = (&'static str, &'static str);
type RedirectList = &'static[RedirectPair];

const REDIRECTS: RedirectList = &[
    ("x-vs-wayland", "https://canartuc.medium.com/x11-vs-wayland-the-40-year-display-server-war-explained-37ac8bb0d720")
];

pub fn router() -> Router {
    Router::new()
        .route("/{*key}", get(handle_redirect_list))
        .route("/", get(Redirect::to("/")))
}

async fn handle_redirect_list(Path(path): Path<String>) -> Redirect {
    Redirect::to(
        REDIRECTS.iter()
        .find(|(uri, _)| uri == &path)
        .map(|(_, link)| *link)
        .unwrap_or("/404")
    )
}
