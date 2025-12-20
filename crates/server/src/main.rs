mod server;

use tracing::{debug, info};

use realm::config::parse_config;
use server::run;
use storage::{config::Config as DbConfig, db_migrator};

async fn run_migrations(db_config: DbConfig) {
    info!("Running db migrations");
    let res = db_migrator::run_migrations(db_config).await;
    info!("Migrations completed with result: {:?}", res);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // todo: remove the expect here
    let config = parse_config().await.expect("Failed to parse config");

    debug!("Config: {:?}", config);

    run_migrations(config.database_config.clone()).await;

    let context = realm::context::Context::new(config)
        .await
        .expect("Could not create context");

    let _ = run(context).await;
}
