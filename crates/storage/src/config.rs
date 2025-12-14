use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub db_url: String,
    pub max_connections: u32,
    pub migrations_dir: PathBuf,
}
