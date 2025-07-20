use std::{
    net::SocketAddr,
    env,
    time::Duration,
};
use axum::{
    http::{HeaderValue, Method},
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use serde_json::{json, Value};
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
    compression::CompressionLayer,
};
use tower::ServiceBuilder;
use tracing::{info,warn};
use tracing_subscriber::prelude::*;

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
    version: String,
    uptime: String,
    timestamp: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing()?;

    let state = AppState {
        // Initialize your application state here
    };

    let app = create_router(state);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;

   serve(app, port).await?;

   Ok(())
}

#[derive(Clone)]
struct AppState {
    // 
}

fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(CompressionLayer::new())
                .layer(create_cors_layer())
        )
        .with_state(state)
}

fn create_cors_layer() -> CorsLayer {
    let origins = env::var("CORS_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000,http://localhost:3001".to_string())
        .split(",")
        .map(|s| s.trim().parse::<HeaderValue>())
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

fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
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

async fn serve(app: Router, port: u16)-> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("Listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> Json<Value> {
    Json(json!({
        "message": "Welcome to the Axum API Template",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn health_check() -> Json<HealthCheckResponse> {
    let response = HealthCheckResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime: "0s".to_string(), // Placeholder for uptime logic
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    Json(response)
}