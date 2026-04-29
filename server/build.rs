use std::{fs, io::Read};

use snafu::prelude::*;
use build_rs::{input, output};
use toml::{self, Value, map::Map};

const CONFIG_FILE: &str = "./server.toml";

const DEFAULT_CFG: &str =
"[server]
PORT = 4000
ASSETS = \"dist/\"
";


fn main() -> Result<(), BuildError> {
    let cfg = get_build_config()?;
    handle_env_vars(cfg)?;

    Ok(())
}

fn get_build_config() -> Result<toml::Table, BuildError> {
    let cfg = ensure_config_file()?;
    let cfg = parse_config_file(cfg)?;
    extract_server_config(cfg)
}

fn handle_env_vars(cfg: Map<String, Value>) -> Result<(), BuildError> {

    let port_source = if is_debug_build() { "DEV_PORT" } else { "PORT" };

    let port = cfg.get(port_source).ok_or(BuildError::needs_key(port_source))?;
    let assets = cfg.get("ASSETS").ok_or(BuildError::needs_key("ASSETS"))?;

    output::rustc_env("PORT", &port.to_string());
    output::rustc_env("ASSETS", &assets.to_string());

    Ok(())
}

fn is_debug_build() -> bool {
    input::cargo_cfg_debug_assertions()
}

fn parse_config_file(mut file: fs::File) -> Result<toml::Table, BuildError> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents.parse::<toml::Table>()
        .map_err(|e| BuildError::from(e))
}

fn extract_server_config(cfg: toml::Table) -> Result<toml::Table, BuildError> {
    cfg.get("server")
        .ok_or(BuildError::FormatFailure { reason: "Missing [server] section".into() })?
        .as_table().ok_or(BuildError::FormatFailure { reason: "Missing [server] section".into() }).cloned()
}

fn ensure_config_file() -> Result<fs::File, std::io::Error> {
    if !fs::exists(CONFIG_FILE)? {
        fs::write(CONFIG_FILE, DEFAULT_CFG)?;
    }
    fs::File::open(CONFIG_FILE)
}

#[derive(Debug, Snafu)]
enum BuildError {
    #[snafu(display("Could not read build configuration: {reason}"))]
    IoFailure{
        source: std::io::Error,
        reason: String
    },
    #[snafu(display("Config file error: {reason}"))]
    FormatFailure{
        reason: String
    },
    #[snafu(display("Required server config missing: {key}"))]
    RequiredKeyMissing{
        key: String
    }
}
impl From<std::io::Error> for BuildError {
    fn from(e: std::io::Error) -> Self {
        BuildError::IoFailure {
            reason: e.to_string(),
            source: e
        }
    }
}
impl From<toml::de::Error> for BuildError {
    fn from(e: toml::de::Error) -> Self {
        BuildError::FormatFailure {
            reason: e.to_string()
        }
    }
}
impl BuildError {
    fn needs_key(k: &str) -> Self {
        BuildError::RequiredKeyMissing { key: k.into() }
    }
}
