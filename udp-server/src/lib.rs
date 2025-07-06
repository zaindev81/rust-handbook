// lib.rs
use std::net::{UdpSocket, SocketAddr};
use std::io;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// Configuration for the UDP echo server
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Host address to bind to (default: "127.0.0.1")
    pub host: String,
    /// Port to bind to (default: 8080)
    pub port: u16,
    /// Buffer size for receiving messages (default: 1024)
    pub buffer_size: usize,
    /// Maximum allowed message size (default: 8192)
    pub max_message_size: usize,
    /// Enable verbose logging (default: false)
    pub verbose: bool,
    /// Enable statistics reporting (default: false)
    pub stats_enabled: bool,
    /// Statistics reporting interval in seconds (default: 30)
    pub stats_interval: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            buffer_size: 8192,
            max_message_size: 8192,
            verbose: false,
            stats_enabled: false,
            stats_interval: 30,
        }
    }
}

impl ServerConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the host address
    pub fn host<S: Into<String>>(mut self, host: S) -> Self {
        self.host = host.into();
        self
    }
    
    /// Set the port
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    
    /// Set the buffer size
    pub fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }
    
    /// Set the maximum message size
    pub fn max_message_size(mut self, size: usize) -> Self {
        self.max_message_size = size;
        self
    }
    
    /// Enable verbose logging
    pub fn verbose(mut self, enabled: bool) -> Self {
        self.verbose = enabled;
        self
    }
    
    /// Enable statistics reporting
    pub fn stats(mut self, enabled: bool) -> Self {
        self.stats_enabled = enabled;
        self
    }
    
    /// Set statistics reporting interval
    pub fn stats_interval(mut self, seconds: u64) -> Self {
        self.stats_interval = seconds;
        self
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.buffer_size < 64 || self.buffer_size > 65536 {
            return Err("Buffer size must be between 64 and 65536 bytes".to_string());
        }
        
        if self.max_message_size > 65536 {
            return Err("Max message size cannot exceed 65536 bytes".to_string());
        }
        
        if self.max_message_size > self.buffer_size {
            return Err("Max message size cannot exceed buffer size".to_string());
        }
        
        Ok(())
    }
}

/// Statistics for the UDP echo server
#[derive(Debug, Clone)]
pub struct ServerStats {
    pub messages_received: u64,
    pub messages_sent: u64,
    pub errors: u64,
    pub bytes_received: u64,
    pub bytes_sent: u64,
    pub start_time: Instant,
}

impl ServerStats {
    fn new() -> Self {
        Self {
            messages_received: 0,
            messages_sent: 0,
            errors: 0,
            bytes_received: 0,
            bytes_sent: 0,
            start_time: Instant::now(),
        }
    }
    
    /// Get the uptime of the server
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// Get messages per second received
    pub fn messages_per_second_received(&self) -> f64 {
        let elapsed = self.uptime().as_secs_f64();
        if elapsed > 0.0 {
            self.messages_received as f64 / elapsed
        } else {
            0.0
        }
    }
    
    /// Get messages per second sent
    pub fn messages_per_second_sent(&self) -> f64 {
        let elapsed = self.uptime().as_secs_f64();
        if elapsed > 0.0 {
            self.messages_sent as f64 / elapsed
        } else {
            0.0
        }
    }
}

/// Message handler trait for custom message processing
pub trait MessageHandler: Send + Sync {
    /// Process an incoming message and return the response
    /// If None is returned, no response is sent
    fn handle_message(&self, message: &[u8], from: SocketAddr) -> Option<Vec<u8>>;
    
    /// Called when an error occurs
    fn on_error(&self, error: &str, from: Option<SocketAddr>) {
        eprintln!("Error from {:?}: {}", from, error);
    }
}

/// Default echo message handler
pub struct EchoHandler;

impl MessageHandler for EchoHandler {
    fn handle_message(&self, message: &[u8], _from: SocketAddr) -> Option<Vec<u8>> {
        Some(message.to_vec())
    }
}

/// UDP Echo Server
pub struct UdpEchoServer {
    socket: Arc<UdpSocket>,
    config: ServerConfig,
    stats: Arc<std::sync::Mutex<ServerStats>>,
}

impl UdpEchoServer {
    /// Create a new UDP echo server with the given configuration
    pub fn new(config: ServerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        config.validate()?;
        
        let bind_addr = format!("{}:{}", config.host, config.port);
        let socket = UdpSocket::bind(&bind_addr)?;
        
        Ok(Self {
            socket: Arc::new(socket),
            config,
            stats: Arc::new(std::sync::Mutex::new(ServerStats::new())),
        })
    }
    
    /// Create a new UDP echo server with default configuration
    pub fn default() -> Result<Self, Box<dyn std::error::Error>> {
        Self::new(ServerConfig::default())
    }
    
    /// Get the local address the server is bound to
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }
    
    /// Get a copy of the current statistics
    pub fn stats(&self) -> ServerStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Run the server with the default echo handler
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.run_with_handler(EchoHandler)
    }
    
    /// Run the server with a custom message handler
    pub fn run_with_handler<H: MessageHandler + 'static>(&self, handler: H) -> Result<(), Box<dyn std::error::Error>> {
        let handler = Arc::new(handler);
        
        if self.config.verbose {
            println!("UDP Echo Server starting on {}", self.local_addr()?);
            println!("Configuration: {:?}", self.config);
        }
        
        // Start statistics reporting thread if enabled
        if self.config.stats_enabled {
            let stats_clone = Arc::clone(&self.stats);
            let interval = self.config.stats_interval;
            let verbose = self.config.verbose;
            
            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(interval));
                    let stats = stats_clone.lock().unwrap();
                    
                    if verbose {
                        println!("=== Server Statistics ===");
                        println!("Uptime: {:.2}s", stats.uptime().as_secs_f64());
                        println!("Messages - Received: {}, Sent: {}, Errors: {}", 
                            stats.messages_received, stats.messages_sent, stats.errors);
                        println!("Bytes - Received: {}, Sent: {}", 
                            stats.bytes_received, stats.bytes_sent);
                        println!("Rate - RX: {:.2} msg/s, TX: {:.2} msg/s", 
                            stats.messages_per_second_received(), 
                            stats.messages_per_second_sent());
                        println!("========================");
                    }
                }
            });
        }
        
        // Main server loop
        let mut buffer = vec![0; self.config.buffer_size];
        
        loop {
            match self.socket.recv_from(&mut buffer) {
                Ok((size, src_addr)) => {
                    // Update statistics
                    {
                        let mut stats = self.stats.lock().unwrap();
                        stats.messages_received += 1;
                        stats.bytes_received += size as u64;
                    }
                    
                    if self.config.verbose {
                        println!("Received {} bytes from {}", size, src_addr);
                    }
                    
                    // Check message size limit
                    if size > self.config.max_message_size {
                        let error_msg = format!("Message too large: {} bytes (max: {})", 
                            size, self.config.max_message_size);
                        handler.on_error(&error_msg, Some(src_addr));
                        
                        let mut stats = self.stats.lock().unwrap();
                        stats.errors += 1;
                        continue;
                    }
                    
                    // Process message with handler
                    match handler.handle_message(&buffer[..size], src_addr) {
                        Some(response) => {
                            match self.socket.send_to(&response, src_addr) {
                                Ok(sent_size) => {
                                    let mut stats = self.stats.lock().unwrap();
                                    stats.messages_sent += 1;
                                    stats.bytes_sent += sent_size as u64;
                                    
                                    if self.config.verbose {
                                        println!("Sent {} bytes to {}", sent_size, src_addr);
                                    }
                                }
                                Err(e) => {
                                    handler.on_error(&format!("Failed to send response: {}", e), Some(src_addr));
                                    let mut stats = self.stats.lock().unwrap();
                                    stats.errors += 1;
                                }
                            }
                        }
                        None => {
                            if self.config.verbose {
                                println!("No response generated for message from {}", src_addr);
                            }
                        }
                    }
                }
                Err(e) => {
                    handler.on_error(&format!("Failed to receive data: {}", e), None);
                    let mut stats = self.stats.lock().unwrap();
                    stats.errors += 1;
                    
                    // Small delay to prevent busy-waiting on persistent errors
                    thread::sleep(Duration::from_millis(10));
                }
            }
        }
    }
    
    /// Run the server in a separate thread and return a handle
    pub fn spawn(self) -> thread::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
        thread::spawn(move || {
            match self.run() {
                Ok(()) => Ok(()),
                Err(e) => Err(format!("Server error: {}", e).into()),
            }
        })
    }
    
    /// Run the server with a custom handler in a separate thread
    pub fn spawn_with_handler<H: MessageHandler + 'static>(
        self, 
        handler: H
    ) -> thread::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
        thread::spawn(move || {
            match self.run_with_handler(handler) {
                Ok(()) => Ok(()),
                Err(e) => Err(format!("Server error: {}", e).into()),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::UdpSocket;
    use std::time::Duration;
    
    #[test]
    fn test_server_config_builder() {
        let config = ServerConfig::new()
            .host("0.0.0.0")
            .port(9999)
            .buffer_size(2048)
            .verbose(true)
            .stats(true);
        
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 9999);
        assert_eq!(config.buffer_size, 2048);
        assert!(config.verbose);
        assert!(config.stats_enabled);
    }
    
    #[test]
    fn test_server_creation() {
        let config = ServerConfig::new().port(0); // Use port 0 for OS to assign
        let server = UdpEchoServer::new(config).unwrap();
        
        // Should be able to get local address
        assert!(server.local_addr().is_ok());
    }
    
    #[test]
    fn test_echo_functionality() {
        let config = ServerConfig::new().port(0);
        let server = UdpEchoServer::new(config).unwrap();
        let server_addr = server.local_addr().unwrap();
        
        // Start server in background thread
        let _handle = server.spawn();
        
        // Give server time to start
        thread::sleep(Duration::from_millis(100));
        
        // Create client socket
        let client = UdpSocket::bind("127.0.0.1:0").unwrap();
        
        // Send message
        let message = b"Hello, Server!";
        client.send_to(message, server_addr).unwrap();
        
        // Receive echo
        let mut buffer = [0; 1024];
        let (size, _) = client.recv_from(&mut buffer).unwrap();
        
        assert_eq!(&buffer[..size], message);
    }
}