use std::collections::HashMap;

use reqwest;
use tokio;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GetResponse {
    args: HashMap<String, String>,
    headers: Headers,
    origin: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct Headers {
    #[serde(rename = "Accept")]
    accept: String,

    #[serde(rename = "Host")]
    host: String,

    #[serde(rename = "X-Amzn-Trace-Id")]
    x_amzn_trace_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();

    let res = client.get("http://httpbin.org/get")
        .send()
        .await?;

    let body: GetResponse = res.json().await?;

   println!("Response: {:#?}", body);

    println!("\nYour IP: {}", body.origin);
    println!("URL: {}", body.url);

    println!("Args count: {}", body.args.len());
    println!("Header accept: {}", body.headers.accept);
    println!("Header host: {}", body.headers.host);
    println!("Header xAmznTraceId: {}", body.headers.x_amzn_trace_id);

    Ok(())
}