use reqwest;
use tokio;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();

    let json_body = json!({
        "lang": "rust",
        "body": "json",
    });

    let res = client
        .post("http://httpbin.org/post")
        .json(&json_body)
        .send()
        .await?;

    let body: Value = res.json().await?;

    println!("{}", body);
    println!("Full response:\n{}", serde_json::to_string_pretty(&body)?);

    // Access specific fields from the JSON
    if let Some(json_data) = body.get("json") {
        println!("\nSent JSON data: {}", json_data);
    }

    if let Some(url) = body.get("url") {
        println!("URL: {}", url);
    }


    Ok(())

}