use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use thiserror::Error;
use tokio::fs;
use tracing::{debug, info};

use storage::config::Config as DatabaseConfig;
#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to read config file because: {0}")]
    ReadConfigFile(#[from] std::io::Error),
    #[error("Failed to parse config file because: {0}")]
    ParseConfig(#[from] toml::de::Error),
    #[error("Failed to read environment variable because: {0}")]
    EnvVar(#[from] std::env::VarError),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "database-config")]
    pub database_config: DatabaseConfig,
}

impl Config {
    pub fn new(config_path: &PathBuf) -> Result<Config, Error> {
        let config_str = std::fs::read_to_string(config_path)?;
        let config = toml::from_str::<Config>(&config_str)?;
        Ok(config)
    }
}

async fn config_path() -> Result<PathBuf, Error> {
    let mut config_file_path = env::var("CARGO_MANIFEST_DIR")?;
    config_file_path.push_str("/../../config.toml");
    let config_file_path = fs::canonicalize(config_file_path).await?;
    Ok(config_file_path)
}

pub async fn parse_config() -> Result<Config, Error> {
    info!("Parsing config");
    let config_file_path = config_path().await?;
    debug!("Config file path: {}", config_file_path.display());
    Config::new(&config_file_path)
}
