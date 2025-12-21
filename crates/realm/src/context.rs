use thiserror::Error;

use crate::config::Config;
use storage::database::Database;

#[derive(Debug, Clone)]
pub struct Context {
    pub db: Database,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl Context {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let db = Database::new(config.database_config.clone()).await?;

        Ok(Self { db })
    }
}
