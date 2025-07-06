mod config;
mod state;
mod models;
mod http;
mod router;
mod handlers;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::{json, Value};

use config::ServerConfig;
use state::AppState;
use router::Router;
use http::{HttpRequest, HttpResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::default();
    let state = AppState::new();

    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;

    println!("🚀 Enhanced JSON API Server running on http://{}:{}", config.host, config.port);
    println!("📋 Available endpoints:");
    println!("   GET /api/users        - List all users");
    println!("   GET /api/user/{{id}}    - Get specific user");
    println!("   GET /api/status       - Server status");
    println!("   GET /api/data         - Sample data");
    println!("   GET /api/health       - Health check");
    println!("   GET /                 - API documentation");

    loop {
        let (socket, addr)  = listener.accept().await?;
        let state_clone = state.clone();
        let config_clone = config.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, state_clone, config_clone).await {
                eprintln!("❌ Error handling connection from {}: {}", addr, e);
            }
        });

    }
}

async fn handle_connection(mut socket: TcpStream, state: AppState, config: ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = vec![0; config.max_buffer_size];
    let bytes_read = socket.read(&mut buffer).await?;


    let raw_request = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("📥 Request: {}", raw_request.lines().next().unwrap_or(""));

    let request = match HttpRequest::parse(&raw_request) {
        Ok(req) => req,
        Err(e) => {
            let error_response = json!({
                "error": {
                    "code": 400,
                    "message": format!("Failed to parse request: {}", e),
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }
            });
            let response = HttpResponse::with_json(400, &error_response)?;
            socket.write_all(&response.to_bytes()).await?;
            return Ok(());
        }
    };

    let router = Router::new(state);
    let response = router.route(&request);

    socket.write_all(&response.to_bytes()).await?;
    println!("📤 Response: {} {}", response.status_code, 
             if response.status_code == 200 { "OK" } else { "Error" });

    Ok(())
}
