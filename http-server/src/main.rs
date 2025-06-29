use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("JSON API Server running on http://127.0.0.1:3000");
    println!("Try these endpoints:");
    println!("  GET /api/users");
    println!("  GET /api/user/1");    println!("  GET /api/status");
    println!("  GET /api/data");

    loop {
        let (socket, addr)  = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 4096];
    let bytes_read = socket.read(&mut buffer).await?;

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request_line = request.lines().next().unwrap_or("");
    
    println!("Request: {}", request_line);

    let info = json!({
        "version": "1.0",
        "description": "This is a simple JSON API server.",
        "endpoints": [
            { "method": "GET", "path": "/api/users", "description": "Get all users" },
            { "method": "GET", "path": "/api/user/{id}", "description": "Get user by ID" },
            { "method": "GET", "path": "/api/status", "description": "Get server status" },
            { "method": "GET", "path": "/api/data", "description": "Get some data" }
        ]
    });
    send_json_response(&mut socket, 200, &info).await?;

    Ok(())
}

async fn send_json_response(socket: &mut TcpStream, status_code: u16,  json_data: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string_pretty(json_data)?;
    let status_text = match status_code {
        200 => "200 OK",
        201 => "201 Created",
        400 => "400 Bad Request",
        404 => "404 Not Found",
        500 => "500 Internal Server Error",
        _ => "Unknown Status",
    };

    let response = format!(
        "HTTP/1.1 {} {}\r\n\
         Content-Type: application/json; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Access-Control-Allow-Methods: GET, POST, PUT, DELETE\r\n\
         Access-Control-Allow-Headers: Content-Type\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        status_code, status_text, json_string.len(), json_string
    );

    socket.write_all(response.as_bytes()).await?;
    Ok(())
}