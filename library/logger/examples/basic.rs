use std::env;
use tracing::{info, warn, span, Level, error};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging()?;

    info!("Starting application");

    run_application()?;

    info!("Application finished successfully");

    Ok(())
}

fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            format!("{}=trace", env!("CARGO_CRATE_NAME")).into()
        });

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_level(true)
        )
        .try_init()
        .map_err(|e| format!("Failed to initialize logging: {}", e))?;

    info!("This is an info log");
    warn!("This is a warning");

    Ok(())
}

fn run_application() -> Result<(), Box<dyn std::error::Error>> {
    let app_span = span!(Level::INFO, "application_main");
    let _enter = app_span.enter();

    info!("Running main application logic");

    process_user_data(42)?;
    process_configuration()?;

    Ok(())
}

fn process_user_data(user_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let span = span!(Level::INFO, "process_user_data", user_id = user_id);
    let _enter = span.enter();

    info!("Processing data for user {}", user_id);
    if user_id == 0 {
        error!("Invalid user ID: {}", user_id);
        return Err("User ID cannot be zero".into());
    }

    Ok(())
}

fn process_configuration() -> Result<(), Box<dyn std::error::Error>> {
    let span = span!(Level::INFO, "process_configuration");
    let _enter = span.enter();

    info!("Loading configuration");

    match env::var("CONFIG_PATH") {
        Ok(path) => {
            info!("Using config path: {}", path);
        }
        Err(_) => {
            warn!("CONFIG_PATH not set, using default configuration");
        }
    }

    info!("Configuration processing completed");
    Ok(())
}