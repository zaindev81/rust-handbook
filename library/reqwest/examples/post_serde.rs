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

    let mut body = HashMap::new();
    body.insert("lang", "rust");
    body.insert("body", "json");

    let res = client.post("https://httpbin.org/post")
        .json(&body)
        .send()
        .await?;

    let res_json: PostResponse = res.json().await?;
    // let res_json = res.json::<PostResponse>().await?;
    println!("Response: {:?}", res_json);
    println!("Response: {:#?}", res_json);

    if let Some(json) = res_json.json {
        println!("Language: {}", json.lang);
        println!("Body: {}", json.body);
    }

    println!("URL: {}", res_json.url);
    println!("Origin: {}", res_json.origin);

    println!("Content-Type header: {}", res_json.headers.content_type);
    println!("Accept: {}", res_json.headers.accept);
    println!("Content-Length: {}", res_json.headers.content_length);
    println!("Content-Type: {}", res_json.headers.content_type);
    println!("Host: {}", res_json.headers.host);
    println!("X-Amzn-Trace-Id: {}", res_json.headers.x_amzn_trace_id);

    Ok(())
}