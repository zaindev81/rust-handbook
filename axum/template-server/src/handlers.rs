use std::{env, sync::atomic::Ordering};
use axum::{
    extract::{State, Path},
    response::Json,
};
use serde_json::{json, Value};
use tracing::{info, warn};

use crate::state::AppState;
use crate::models::{HealthCheckResponse, CounterResponse, ErrorResponse};

pub async fn root() -> Json<Value> {
    Json(json!({
        "message": "Welcome to the Axum API Template",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

pub async fn health_check(State(state): State<AppState>) -> Json<HealthCheckResponse> {
    let uptime = state.start_time.elapsed();
    let uptime_str = format!(
        "{}d {}h {}m {}s",
        uptime.as_secs() / 86400,
        (uptime.as_secs() % 86400) / 3600,
        (uptime.as_secs() % 3600) / 60,
        uptime.as_secs() % 60
    );
    let response = HealthCheckResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime: uptime_str,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    Json(response)
}

pub async fn get_counter(State(state): State<AppState>) -> Json<CounterResponse> {
    let count = state.counter.load(Ordering::Relaxed);
    Json(CounterResponse {
        count,
        message: format!("Current counter value is {}", count),
    })
}

pub async fn increment_counter(State(state): State<AppState>) -> Json<CounterResponse> {
    let count = state.counter.fetch_add(1, Ordering::Relaxed) + 1;
    Json(CounterResponse {
        count,
        message: format!("Counter incremented to {}", count),
    })
}

pub async fn set_counter(
    State(state): State<AppState>,
    Path(value): Path<String>
) -> Result<Json<CounterResponse>, ErrorResponse> {
    let parsed_value = match value.parse::<u64>() {
        Ok(v) => v,
        Err(_) => {
            warn!("Invalid counter value provided: {}", value);
            return Err(ErrorResponse::new(
                &format!("'{}' is not a valid number", value),
                "INVALID_VALUE"
            ));
        }
    };

    if parsed_value > 1000000 {
        warn!("Counter value too high: {}", parsed_value);
        return Err(ErrorResponse::new(
            "Counter value must be between 0 and 1,000,000",
            "INVALID_VALUE"
        ));
    }

    let old_value = state.counter.swap(parsed_value, Ordering::Relaxed);
    info!("Counter set from {} to {}", old_value, parsed_value);

    Ok(Json(CounterResponse {
        count: parsed_value,
        message: format!("Counter set from {} to {}", old_value, parsed_value),
    }))
}

pub async fn handle_404() -> ErrorResponse {
    warn!("404 - Endpoint not found");
    ErrorResponse::new(
        "The requested endpoint was not found",
        "NOT_FOUND"
    )
}