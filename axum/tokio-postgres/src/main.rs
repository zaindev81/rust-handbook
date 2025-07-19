use std::net::SocketAddr;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use sqlx::postgres::{PgPoolOptions, PgPool};
use sqlx::query_as;
use tracing::{info, debug};
use tracing_subscriber::prelude::*;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let pool = init_database().await?;

    let app_state = AppState { db_pool: pool };

    let app = create_route(app_state);

    serve(app, 3000).await?;
    
    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    info!("Tracing initialized");
}

async fn init_database() -> Result<PgPool, sqlx::Error> {
    debug!("Initializing database connection pool");
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost/test".to_string());
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    debug!("Database connection pool created");

    Ok(pool)
}

fn create_route(state: AppState) -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/health", get(health_check))
        .route("/db-health", get(db_health))
        .with_state(state)
}

async fn serve(app: Router, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("Listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn hello() -> Json<Value> {
    Json(json!({
        "message": "Hello, World!"
    }))
}

async fn db_health(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    debug!("Executing database test query");

    let row: (i64,) = query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(json!({
        "result": row.0
    })))
}

async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}