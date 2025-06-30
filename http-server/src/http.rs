use std::collections::HashMap;
use serde_json::Value;

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
}