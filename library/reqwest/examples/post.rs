use reqwest;
use tokio;
use serde_json::{json, Value, to_string_pretty};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();

    let json_body = json!({
        "lang": "rust",
        "body": "json",
    });

    let res = client
        .post("https://httpbin.org/post")
        .json(&json_body)
        .send()
        .await?;

    let res_json: Value = res.json().await?;

    println!("{}", res_json);
    println!("Full response:\n{}", to_string_pretty(&res_json)?);

    if let Some(json_data) = res_json.get("json") {
        println!("\nSent JSON data: {}", json_data);
    }

    if let Some(url) = res_json.get("url") {
        println!("URL: {}", url);
    }

    if let Some(headers) = res_json.get("headers") {
        println!("headers: {}", headers);
    }

    Ok(())
}