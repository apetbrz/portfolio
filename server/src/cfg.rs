use std::{collections::HashMap, path::PathBuf};

use config::{Config, ConfigError};
use reqwest::Url;
use serde::Deserialize;

pub fn init_cfg() -> Result<AppConfig, ConfigError> {
    let mut cfg: AppConfig = Config::builder()
        //.add_source(config::File::with_name("config").required(false))
        .add_source(config::Environment::default())
        .build()?
        .try_deserialize()?;

    cfg.blog_server = cfg.blog_server.take_if(|url| {
        Url::parse(url).is_ok()
    });


    Ok(cfg)
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub dev_port: Option<u16>,
    pub assets: PathBuf,
    pub blog_server: Option<String>,
    pub redirects: Option<HashMap<String, String>>
}

#[cfg(test)]
#[serial_test::serial]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn env_cfg() {
        env::set_var("PORT", "4000");
        env::set_var("ASSETS", "/srv/dist");
        env::set_var("REDIRECTS.TEST", "http://localhost/");

        let cfg = init_cfg().expect("failed to form config");

        assert_eq!(cfg.port, 4000);
        assert_eq!(cfg.assets, PathBuf::from("/srv/dist"));
        assert_eq!(cfg.redirects.unwrap().get("test").expect("key 'test' missing from cfg"),"http://localhost/");
    }

    #[test]
    fn valid_blog() {
        env::set_var("PORT", "4000");
        env::set_var("ASSETS", "/srv/dist");
        env::set_var("BLOG_SERVER", "http://localhost:3123");

        let cfg = init_cfg().expect("failed to form config");

        assert_eq!(cfg.blog_server, Some("http://localhost:3123".into()));
    }

    #[test]
    fn invalid_blog() {
        env::set_var("PORT", "4000");
        env::set_var("ASSETS", "/srv/dist");
        env::set_var("BLOG_SERVER", "256.1.1.1");

        let cfg = init_cfg().expect("failed to form config");

        assert_eq!(cfg.blog_server, None);
    }
}
