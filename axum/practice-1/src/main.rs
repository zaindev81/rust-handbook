use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use tracing::debug;
use tracing_subscriber::prelude::*;

struct AppState {
    users: Mutex<Vec<User>>,
    next_id: Mutex<u32>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            users: Mutex::new(Vec::new()),
            next_id: Mutex::new(1), // Start from 1 instead of 0
        }
    }
}

#[derive(Deserialize)]
struct HelloQuery {
    msg: Option<String>,
}

#[derive(Clone, Serialize)]
struct User {
    id: u32,
    username: String,
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct CreateUserResponse {
    ok: bool,
    user: Option<User>,
}

#[derive(Serialize)]
struct UsersResponse {
    users: Vec<User>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = Arc::new(AppState::default());

    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/hello/{name}", get(hello_path))
        .route("/hello/query", get(hello_query))
        .route("/users", post(create_user).get(list_users))
        .route("/users/{id}", get(get_user))
        .route("/echo", post(echo))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Root!"
}

async fn hello() -> &'static str {
    "Hello World!"
}

async fn hello_path(Path(name): Path<String>) -> (StatusCode, String) {
    let msg = format!("Hello, {}!", name);
    (StatusCode::OK, msg)
}

async fn hello_query(Query(params): Query<HelloQuery>) -> (StatusCode, String) {
    let msg = params.msg.unwrap_or_else(|| "Msg Hello World!".to_string());
    (StatusCode::OK, msg)
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<CreateUserResponse>) {
    if payload.username.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(CreateUserResponse { ok: false, user: None }),
        );
    }

    // Generate new ID
    let mut next_id = state.next_id.lock().await;
    let user_id = *next_id;
    *next_id += 1;
    drop(next_id); // Release the lock early

    // Create the user
    let user = User {
        id: user_id,
        username: payload.username,
    };

    // Add to users list
    let mut users = state.users.lock().await;
    users.push(user.clone());

    (
        StatusCode::CREATED,
        Json(CreateUserResponse {
            ok: true,
            user: Some(user),
        }),
    )
}

async fn list_users(State(state): State<Arc<AppState>>) -> (StatusCode, Json<UsersResponse>) {
    let users = state.users.lock().await;
    let response = UsersResponse {
        users: users.clone(),
    };
    (StatusCode::OK, Json(response))
}

async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u32>,
) -> (StatusCode, Json<Option<User>>) {
    let users = state.users.lock().await;
    let user = users.iter().find(|u| u.id == id).cloned();

    match user {
        Some(u) => (StatusCode::OK, Json(Some(u))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}

async fn echo(Json(body): Json<JsonValue>) -> (StatusCode, Json<JsonValue>) {
    (StatusCode::OK, Json(body))
}