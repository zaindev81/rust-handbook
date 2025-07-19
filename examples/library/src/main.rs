use std::env;
use tracing::{info, warn, span, Level, error};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use serde::Deserialize;
use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub port: u16,
    pub database: Database,
    pub debug: bool,
    pub log_level: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool_size: u32,
    pub timeout: u64,
}

impl Settings {
    fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        info!("Loading configuration for environment: {}", run_mode);

        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            .set_override("database.url", "postgres://postgres:password@localhost/test")?
            .build()?;
        
        s.try_deserialize()
    }
} 

impl Default for Settings {
    fn default() -> Self {
        Settings {
            port: 8080,
            database: Database {
                url: "postgres://localhost/myapp".to_string(),
                pool_size: 10,
                timeout: 30,
            },
            debug: false,
            log_level: "info".to_string(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting application");

    logger_example();
    let _ = config_example();

    Ok(())
}

fn logger_example() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(fmt::layer())
        .init();

    let span = span!(Level::INFO, "my_span", user_id = 42);
    let _enter = span.enter();

    info!("This is an info log");
    warn!("This is a warning");
}

fn config_example() -> Result<(), Box<dyn std::error::Error>> {
    let settings = match Settings::new() {
        Ok(config) => {
            info!("Configuration loaded successfully");
            config
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            warn!("Using default configuration");
            Settings::default()
        }
    };

    println!("Debug mode: {}", settings.debug);
    println!("Database URL: {}", settings.database.url);

    Ok(())
}
