use std::{
    sync::{atomic::AtomicU64, Arc},
    time::Instant
};

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub counter: Arc<AtomicU64>,
    pub start_time: Instant,
    pub config: Config,
}

impl AppState {
    pub fn new() -> Self {
        let config = Config::from_env().unwrap_or_else(|err| {
            eprintln!("Failed to load config: {}. Using default config.", err);
            Config::default()
        });

        Self {
            counter: Arc::new(AtomicU64::new(config.counter.initial_value)),
            start_time: Instant::now(),
            config,
        }
    }

    pub fn with_config(config: Config) -> Self {
        Self {
            counter: Arc::new(AtomicU64::new(config.counter.initial_value)),
            start_time: Instant::now(),
            config,
        }
    }

    pub fn max_counter_value(&self) -> u64 {
        self.config.counter.max_value
    }
}