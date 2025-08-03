use std::collections::HashMap;

use reqwest;
use tokio;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PostResponse {
    json: Option<JsonData>,
    url: String,
    origin: String,
    headers: Headers,
}

#[derive(Debug, Deserialize)]
struct JsonData {
    lang: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct Headers {
    #[serde(rename = "Accept")]
    accept: String,

    #[serde(rename = "Content-Length")]
    content_length: String,

    #[serde(rename = "Content-Type")]
    content_type: String,

    #[serde(rename = "Host")]
    host: String,

    #[serde(rename = "X-Amzn-Trace-Id")]
    x_amzn_trace_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let res = client.post("http://httpbin.org/post")
        .json(&map)
        .send()
        .await?;

    let body: PostResponse = res.json().await?;
    println!("Response: {:#?}", body);

    if let Some(json) = body.json {
        println!("Language: {}", json.lang);
        println!("Body: {}", json.body);
    }

    println!("URL: {}", body.url);
    println!("Origin: {}", body.origin);
    println!("Content-Type header: {}", body.headers.content_type);
    println!("Accept: {}", body.headers.accept);
    println!("Content-Length: {}", body.headers.content_length);
    println!("Content-Type: {}", body.headers.content_type);
    println!("Host: {}", body.headers.host);
    println!("X-Amzn-Trace-Id: {}", body.headers.x_amzn_trace_id);

    Ok(())
}