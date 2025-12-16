use axum::Router;
use thiserror::Error;
use tracing::info;

use crate::config::Config;
use service::auth::routes as auth_routes;

#[derive(Debug, Error)]
pub enum Error {}

pub async fn run(_config: Config) -> Result<(), Error> {
    info!("Starting server...");

    info!("Registering routes");

    let router = Router::new()
        .route("/", axum::routing::get(|| async { "Hello, world!" }))
        .merge(auth_routes());

    info!("Starting the server");
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
