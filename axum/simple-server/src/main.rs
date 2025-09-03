use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::debug;
use tracing_subscriber::prelude::*;

#[tokio::main] // marks this as the asynchronous entrypoint, running on the Tokio runtime (needed for async code).
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            // reads logging level from env var RUST_LOG.
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer()) // makes logs human-readable.
        .init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
// the string lives for the entire lifetime of the program (stored in the program’s binary).
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
// allows JSON → struct conversion.
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
// allows struct → JSON conversion.
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}