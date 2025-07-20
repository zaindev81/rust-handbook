use std::net::SocketAddr;
use axum::{
    http::{HeaderValue, Method},
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use tower_http::cors::{CorsLayer};
use tracing::info;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    init_tracing();

    let app = Router::new()
        .route("/", get(root))
        .layer(create_cors_layer());

   serve(app, 3000).await;
}

fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([
            "Content-Type".parse().unwrap(),
            "Authorization".parse().unwrap(),
        ])
        .max_age(std::time::Duration::from_secs(3600))
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

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<Value> {
    Json(json!({
        "message": "Hello, World!"
    }))
}