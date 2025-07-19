use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    net::SocketAddr,
};

use axum::{
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use tracing::{info, debug};
use tracing_subscriber::prelude::*;
use time_library::Timestamp;

#[tokio::main]
async fn main() {
    init_tracing();

    let state = AppState::default();

    let app = Router::new()
        .route("/", get(hello));
        
    serve(app).await;
}

async fn serve(app: Router) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
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

async fn hello() -> Json<Value> {
    Json(json!({
        "message": "Hello, world!"
    }))
}

#[derive(Default, Clone)]
struct AppState {
    next_id: Arc<AtomicU64>,
    users: Arc<Mutex<HashMap<u64, User>>>,
}

#[derive(Serialize, Clone)]
struct User {
    id: u64,
    name: String,   
    created_at: Timestamp,
}

mod time_library {
    use std::sync::atomic::{AtomicU64, Ordering};

    use serde::Serialize;

    #[derive(Serialize, Clone)]
    pub struct Timestamp(u64);

    impl Timestamp {
        pub fn now() -> Result<Self, Error> {
            static COUNTER: AtomicU64 = AtomicU64::new(0);

            // Fail on every third call just to simulate errors
            if COUNTER.fetch_add(1, Ordering::SeqCst) % 3 == 0 {
                Err(Error::FailedToGetTime)
            } else {
                Ok(Self(1337))
            }
        }
    }

    #[derive(Debug)]
    pub enum Error {
        FailedToGetTime,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "failed to get time")
        }
    }
}