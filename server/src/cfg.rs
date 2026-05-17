#[derive(Clone)]
pub struct ServerConfig {
    pub markdown_cache_server: String
}
impl ServerConfig {
    pub fn new(md_server: String) -> Self {
        ServerConfig {
            markdown_cache_server: md_server
        }
    }
}
