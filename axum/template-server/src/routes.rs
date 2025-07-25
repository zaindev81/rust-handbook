use std::time::Duration;
use axum::{
    routing::{get, post},
    Router,
};
use tower::{ServiceBuilder};
use tower_http::{
    trace::TraceLayer,
    timeout::TimeoutLayer,
    compression::CompressionLayer,
};

use crate::state::AppState;
use crate::handlers::{root, health_check, get_counter, increment_counter, set_counter, handle_404};
use crate::middleware::create_cors_layer;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/counter", get(get_counter))
        .route("/counter/increment", post(increment_counter))
        .route("/counter/set/{value}", post(set_counter))
        .fallback(handle_404)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(CompressionLayer::new())
                .layer(create_cors_layer(&state.config))
        )
        .with_state(state)
}