#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_buffer_size: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            max_buffer_size: 1024 * 1024, // 1 MB
        }
    }
}

impl ServerConfig {
    pub fn new(host: &str, port: u16) -> Self {
        ServerConfig {
            host: host.to_string(),
            port,
            max_buffer_size: 1024 * 1024, // 1 MB
        }
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.max_buffer_size = size;
        self
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}