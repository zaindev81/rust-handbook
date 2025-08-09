use axum::{
    routing::get,
    Router,
};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tracing::{info, debug};
use tracing_subscriber::prelude::*;

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

   // Allow bursts with up to five requests per IP address
   // and replenishes one element every two seconds
   // We Box it because Axum 0.6 requires all Layers to be Clone
   // and thus we need a static reference to it
   let governor_conf = Arc::new(
       GovernorConfigBuilder::default()
           .per_second(2)
           .burst_size(5)
           .finish()
           .unwrap(),
   );

   let governor_limiter = governor_conf.limiter().clone();
   let interval = Duration::from_secs(60);
   // a separate background task to clean up
   std::thread::spawn(move || {
       loop {
           std::thread::sleep(interval);
           info!("rate limiting storage size: {}", governor_limiter.len());
           governor_limiter.retain_recent();
       }
   });

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(hello))
        .layer(GovernorLayer{
            config: governor_conf,
        });

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()) // important
       .await
       .unwrap();
}

// basic handler that responds with a static string
async fn hello() -> &'static str {
   "Hello world"
}
