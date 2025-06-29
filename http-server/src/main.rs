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
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    println!("Request parts: {:?}", parts);

    if parts.len() < 2 {
        send_json_error(&mut socket, 400, "Bad Request").await?;
        return Ok(());
    }

    let method = parts[0];
    let path = parts[1];

    match (method, path) {
        ("GET", "/api/users") => {
            handle_get_users(&mut socket).await?;
        },
        ("GET", path) if path.starts_with("/api/user/") => {
            handle_get_user(&mut socket, &path[11..]).await?;
        },
        ("GET", "/api/status") => {
            handle_get_status(&mut socket).await?;
        },
        ("GET", "/api/data") => {
            handle_get_data(&mut socket).await?;
        },
        _ => {
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
        }
    }

    Ok(())
}

async fn handle_get_users(socket: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let users = json!({
        "users": [
            {
                "id": 1,
                "name": "Alice Johnson",
                "email": "alice@example.com",
                "role": "admin",
                "active": true
            },
            {
                "id": 2,
                "name": "Bob Smith",
                "email": "bob@example.com",
                "role": "user",
                "active": true
            },
            {
                "id": 3,
                "name": "Charlie Brown",
                "email": "charlie@example.com",
                "role": "user",
                "active": false
            }
        ],
        "total": 3
    });
    send_json_response(socket, 200, &users).await
}

async fn handle_get_user(socket: &mut TcpStream, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    match user_id.parse::<u32>() {
        Ok(id) => {
            let user = match id {
                1 => json!({
                    "id": 1,
                    "name": "Alice Johnson",
                    "email": "alice@example.com",
                    "role": "admin",
                    "active": true,
                    "created_at": "2023-01-15T10:30:00Z"
                }),
                2 => json!({
                    "id": 2,
                    "name": "Bob Smith",
                    "email": "bob@example.com",
                    "role": "user",
                    "active": true,
                    "created_at": "2023-02-20T14:15:00Z"
                }),
                3 => json!({
                    "id": 3,
                    "name": "Charlie Brown",
                    "email": "charlie@example.com",
                    "role": "user",
                    "active": false,
                    "created_at": "2023-03-10T09:45:00Z"
                }),
                _ => {
                    send_json_error(socket, 404, "User not found").await?;
                    return Ok(());
                }
            };
            send_json_response(socket, 200, &user).await
        },
        Err(_) => {
            send_json_error(socket, 400, "Invalid user ID").await
        }
    }
}

async fn handle_get_status(socket: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let status = json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "server": {
            "name": "rust-tcp-json-server",
            "version": "1.0.0"
        },
        "metrics": {
            "total_requests": 1247,
            "active_connections": 5,
            "memory_usage_mb": 45.2
        }
    });
    send_json_response(socket, 200, &status).await
}

async fn handle_get_data(socket: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let data = json!({
        "data": {
            "numbers": [1, 2, 3, 4, 5],
            "strings": ["hello", "world", "rust"],
            "nested": {
                "key1": "value1",
                "key2": {
                    "subkey": "subvalue",
                    "array": [true, false, null]
                }
            }
        },
        "metadata": {
            "generated_at": chrono::Utc::now().to_rfc3339(),
            "format": "json",
            "encoding": "utf-8"
        }
    });
    send_json_response(socket, 200, &data).await
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

async fn send_json_error(socket: &mut TcpStream, status_code: u16, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let error_json = json!({
        "error": {
            "code": status_code,
            "message": message,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }
    });

    send_json_response(socket, status_code, &error_json).await
}