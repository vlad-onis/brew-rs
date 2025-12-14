use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::info;

use crate::config::Config;

pub async fn run_migrations(config: Config) -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.db_url)
        .await?;

    info!("Migrations starting with predefined, not configurable directory ./migrations");

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}
