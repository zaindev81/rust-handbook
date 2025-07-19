use tracing::{info, warn, span, Level};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(fmt::layer())
        .init();

    let span = span!(Level::INFO, "my_span", user_id = 42);
    let _enter = span.enter();

    info!("This is an info log");
    warn!("This is a warning");
}
