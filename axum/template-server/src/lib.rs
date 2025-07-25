pub mod handlers;
pub mod models;
pub mod routes;
pub mod state;
pub mod middleware;
pub mod config;

use std::{env, net::SocketAddr};
use axum::Router;
use tracing::info;
use tracing_subscriber::prelude::*;

pub use state::AppState;
pub use routes::create_router;
pub use config::Config;

pub fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Tracing initialized");
    Ok(())
}

pub fn create_app_with_config(config: Config) -> Router {
    let state = AppState::with_config(config);
    create_router(state)
}

pub async fn serve(app: Router, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
     let addr: SocketAddr = config.server_addr().parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("Listening on http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

/// Run the application with default configuration
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing()?;

    let config = Config::from_env()?;
    let app = create_app_with_config(config.clone());

    serve(app, &config).await
}