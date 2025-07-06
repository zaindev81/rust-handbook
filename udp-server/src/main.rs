use std::net::UdpSocket;
use std::io;

fn main() -> io::Result<()> {
    // Bind to localhost on port 8080
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    println!("UDP Echo Server listening on 127.0.0.1:8080");
    
    // Buffer to store incoming data
    let mut buf = [0; 1024];
    
    loop {
        // Receive data from client
        match socket.recv_from(&mut buf) {
            Ok((size, src_addr)) => {
                println!("Received {} bytes from {}: {}", 
                    size, 
                    src_addr, 
                    String::from_utf8_lossy(&buf[..size])
                );
                
                // Echo the data back to the sender
                match socket.send_to(&buf[..size], src_addr) {
                    Ok(sent_size) => {
                        println!("Echoed {} bytes back to {}", sent_size, src_addr);
                    }
                    Err(e) => {
                        eprintln!("Failed to send response: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to receive data: {}", e);
            }
        }
    }
}