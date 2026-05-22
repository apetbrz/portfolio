use std::{collections::HashMap, path::PathBuf};

use config::{Config, ConfigError};
use reqwest::Url;
use serde::Deserialize;

pub fn init_cfg() -> Result<AppConfig, ConfigError> {
    let cfg: AppConfig = Config::builder()
        .add_source(env_cfg()?)
        .add_source(file_cfg("config")?)
        .add_source(file_cfg("redirects")?)
        .build()?
        .try_deserialize()?;

    Ok(clean_invalid_urls(cfg))
}

fn clean_invalid_urls(cfg: AppConfig) -> AppConfig {
    AppConfig {
        port: cfg.port,
        dev_port: cfg.dev_port,
        assets: cfg.assets,
        blog_server: cfg.blog_server.clone().take_if(|url| Url::parse(url).is_ok()),
        redirects: cfg.redirects.map(|map| {
            map.into_iter().filter(|(_, v)| Url::parse(v).is_ok()).collect()
        })
    }
}

fn env_cfg() -> Result<Config, ConfigError> {
    Config::builder()
        .add_source(config::Environment::default())
        .build()
}

fn file_cfg(file_base_name: &str) -> Result<Config, ConfigError> {
    Config::builder()
        .add_source(config::File::with_name(file_base_name).required(false))
        .build()
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub dev_port: Option<u16>,
    pub assets: PathBuf,
    pub blog_server: Option<String>,
    pub redirects: Option<HashMap<String, String>>,
}

#[cfg(test)]
mod test {
    use serial_test::serial;

    use super::*;
    use std::env::{set_var, remove_var};

    fn conf(c: Result<Config, ConfigError>) -> AppConfig {
        c.expect("failed to form config")
            .try_deserialize()
            .expect("failed to deserialize config")
    }

    #[test]
    #[serial]
    fn env_required_keys() {
        set_var("PORT", "4000");
        set_var("ASSETS", "/srv/dist");

        let cfg: AppConfig = clean_invalid_urls(conf(env_cfg()));

        assert_eq!(cfg.port, 4000);
        assert_eq!(cfg.assets, PathBuf::from("/srv/dist"));

        remove_var("PORT");
        remove_var("ASSETS");
    }

    #[test]
    #[serial]
    fn testcfg_toml() {
        assert!(std::fs::read("config.toml").is_ok());

        let cfg: AppConfig = clean_invalid_urls(conf(file_cfg("testcfg")));

        assert_eq!(cfg.port, 4000);
        assert_eq!(cfg.dev_port, Some(3000));
        assert_eq!(cfg.assets, PathBuf::from("/srv/dist"));
        assert_eq!(cfg.blog_server, Some("http://localhost/".into()), "blog_server link incorrect");
        assert_eq!(cfg.redirects.unwrap().get("test_link"), Some(&String::from("http://localhost/")), "redirects link incorrect");
    }

    #[test]
    #[serial]
    fn env_blog_input_valid() {
        set_var("PORT", "4000");
        set_var("ASSETS", "/srv/dist");
        set_var("BLOG_SERVER", "http://localhost:3123");

        let cfg: AppConfig = clean_invalid_urls(conf(env_cfg()));

        assert_eq!(cfg.blog_server, Some("http://localhost:3123".into()));

        remove_var("PORT");
        remove_var("ASSETS");
        remove_var("BLOG_SERVER");
    }

    #[test]
    #[serial]
    fn env_blog_input_invalid() {
        set_var("PORT", "4000");
        set_var("ASSETS", "/srv/dist");
        set_var("BLOG_SERVER", "256.1.1.1");

        let cfg: AppConfig = clean_invalid_urls(conf(env_cfg()));

        assert_eq!(cfg.blog_server, None);

        remove_var("PORT");
        remove_var("ASSETS");
        remove_var("BLOG_SERVER");
    }

    #[test]
    #[serial]
    fn env_redirect_input_valid() {
        set_var("PORT", "4000");
        set_var("ASSETS", "/srv/dist");
        set_var("REDIRECTS.TEST_LINK", "http://localhost/");

        let cfg: AppConfig = clean_invalid_urls(conf(env_cfg()));

        assert_eq!(cfg.redirects.unwrap().get("test_link"), Some(&String::from("http://localhost/")));

        remove_var("PORT");
        remove_var("ASSETS");
        remove_var("REDIRECTS.TEST_LINK");
    }

    #[test]
    #[serial]
    fn env_redirect_input_invalid() {
        set_var("PORT", "4000");
        set_var("ASSETS", "/srv/dist");
        set_var("REDIRECTS.TEST_LINK", "256.1.1.1");

        let cfg: AppConfig = clean_invalid_urls(conf(env_cfg()));

        assert_eq!(cfg.redirects.unwrap().get("test_link"), None);

        remove_var("PORT");
        remove_var("ASSETS");
        remove_var("REDIRECTS.TEST_LINK");
    }
}
