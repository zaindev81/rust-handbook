use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::{json, Value};
use std::collections::HashMap;

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
    Ok(())
}