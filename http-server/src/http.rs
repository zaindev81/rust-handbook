use std::collections::HashMap;
use serde_json::{json, Value};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status_code: u16) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json; charset=utf-8".to_string());
        headers.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());
        headers.insert("Access-Control-Allow-Methods".to_string(), "GET, POST, PUT, DELETE".to_string());
        headers.insert("Access-Control-Allow-Headers".to_string(), "Content-Type".to_string());
        headers.insert("Connection".to_string(), "close".to_string());

        Self {
            status_code,
            headers,
            body: String::new(),
        }
    }

    pub fn with_json(status_code: u16, json_data: &Value) -> Result<Self, serde_json::Error> {
        let mut response = Self::new(status_code);
        response.body = serde_json::to_string_pretty(&json_data)?;
        response.headers.insert("Content-Length".to_string(), response.body.len().to_string());
        Ok(response)
    }

    pub fn error(status_code: u16, message: &str) -> Result<Self, serde_json::Error> {
        let error_data = json!({
            "error": {
                "code": status_code,
                "message": message,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });
        Self::with_json(status_code, &error_data)
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
}