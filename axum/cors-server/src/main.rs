use std::net::SocketAddr;
use axum::{
    http::{HeaderValue, Method},
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use tower_http::cors::{CorsLayer};

#[tokio::main]
async fn main() {
     let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        // .allow_methods(Any)
        .allow_headers([
            "Content-Type".parse().unwrap(),
            "Authorization".parse().unwrap(),
        ])
        .max_age(std::time::Duration::from_secs(3600));

    let app = Router::new()
        .route("/", get(hello))
        .route("/api", get(api_endpoint))
        .layer(cors);

   serve(app, 3000).await;
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> Json<Value> {
    Json(json!({
        "message": "Hello, World!"
    }))
}

async fn api_endpoint() -> Json<Value> {
    Json(json!({
        "data": "This is a CORS-enabled API endpoint."
    }))
}