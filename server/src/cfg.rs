use reqwest::Url;

#[derive(Clone)]
pub struct ServerConfig {
    pub markdown_cache_server: Url
}
impl ServerConfig {
    pub fn new(md_server: String) -> Self {
        ServerConfig {
            markdown_cache_server: Url::parse(&md_server).unwrap()
        }
    }
}
