use std::net::UdpSocket;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    let server_addr = "127.0.0.1:8080";
    
    println!("UDP Client - Type messages to send to {}", server_addr);
    println!("Type 'quit' to exit");
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let message = input.trim();
        if message == "quit" {
            break;
        }
        
        // Send message
        socket.send_to(message.as_bytes(), server_addr)?;
        
        // Receive response
        let mut buffer = [0; 1024];
        match socket.recv_from(&mut buffer) {
            Ok((size, from)) => {
                let response = std::str::from_utf8(&buffer[..size])?;
                println!("Response from {}: {}", from, response);
            }
            Err(e) => {
                eprintln!("Error receiving response: {}", e);
            }
        }
    }
    
    Ok(())
}