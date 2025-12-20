use sqlx::PgPool;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct Database {
    pub config: Config,
    pub pool: PgPool,
}

impl Database {
    pub async fn new(config: Config) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(&config.db_url).await?;

        Ok(Self { config, pool })
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[tokio::test]
    async fn connection() {
        let config = Config {
            db_url: "postgres://postgres:postgres@0.0.0.0:5432/brewrs".to_string(),
            max_connections: 5,
        };

        let db = Database::new(config).await.unwrap();
    }
}
