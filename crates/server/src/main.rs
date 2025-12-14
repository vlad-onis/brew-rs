mod config;
mod server;

use tracing::{debug, info};

use config::parse_config;
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
    let _ = run(config).await;

    // let router = Router::new()
    //     .route("/", axum::routing::get(|| async { "Hello, world!" }))
    //     .merge(auth_routes());

    // info!("Starting the server");
    // // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, router).await.unwrap();
}
