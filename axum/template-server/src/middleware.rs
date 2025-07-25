use std::time::Duration;
use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;
use tracing::warn;

use crate::config::Config;

pub fn create_cors_layer(config: &Config) -> CorsLayer {
    let origins = config.cors.origins
        .iter()
        .map(|s| s.parse::<HeaderValue>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|_| {
            warn!("Invalid CORS origins, using default");
            vec!["http://localhost:3000".parse().unwrap()]
        });

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
        .allow_headers([
            "Content-Type".parse().unwrap(),
            "Authorization".parse().unwrap(),
            "Accept".parse().unwrap(),
            "Origin".parse().unwrap(),
            "X-Requested-With".parse().unwrap(),
        ])
        .max_age(Duration::from_secs(3600))
}