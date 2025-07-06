use udp_echo_server::{UdpEchoServer, ServerConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple server with default configuration
    let server = UdpEchoServer::default()?;
    println!("Starting echo server on {}", server.local_addr()?);
    server.run()?;
    Ok(())
}