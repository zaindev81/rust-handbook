use std::{
    env, net::SocketAddr,
    sync::{atomic::{AtomicU64, Ordering},Arc},
    time::{Duration, Instant}
};
use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::{Json, IntoResponse, Response},
    routing::{get, post},
    Router,
    extract::{State, Path},
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
struct CounterResponse {
    count: u64,
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: String,
}

impl ErrorResponse {
    fn new(error: &str, code: &str) -> Self {
        Self {
            error: error.to_string(),
            code: code.to_string(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = match self.code.as_str() {
            "INVALID_VALUE" => StatusCode::BAD_REQUEST,
            "NOT_FOUND" => StatusCode::NOT_FOUND,
            "INTERNAL_ERROR" => StatusCode::INTERNAL_SERVER_ERROR,
            "RATE_LIMITED" => StatusCode::TOO_MANY_REQUESTS,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(self)).into_response()
    }
}

#[derive(Clone)]
struct AppState {
    counter: Arc<AtomicU64>,
    start_time: Instant,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing()?;

    let state = AppState {
        counter: Arc::new(AtomicU64::new(0)),
        start_time: Instant::now(),
    };

    let app = create_router(state);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;

   serve(app, port).await?;

   Ok(())
}

fn create_router(state: AppState) -> Router {
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

async fn health_check(State(state): State<AppState>) -> Json<HealthCheckResponse> {
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

async fn get_counter(State(state): State<AppState>) -> Json<CounterResponse> {
    let count = state.counter.load(Ordering::Relaxed);
    Json(CounterResponse {
        count,
        message: format!("Current counter value is {}", count),
    })
}

async fn increment_counter(State(state): State<AppState>) -> Json<CounterResponse> {
    let count = state.counter.fetch_add(1, Ordering::Relaxed) + 1;
    Json(CounterResponse {
        count,
        message: format!("Counter incremented to {}", count),
    })
}

async fn set_counter(
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

async fn handle_404() -> ErrorResponse {
    warn!("404 - Endpoint not found");
    ErrorResponse::new(
        "The requested endpoint was not found",
        "NOT_FOUND"
    )
}