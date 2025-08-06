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

    let mut query_params = HashMap::new();
    query_params.insert("lang", "rust");

    let res = client.get("https://httpbin.org/get")
        .query(&query_params)
        .send()
        .await?;

    let res_json: GetResponse = res.json().await?;
    // let res_json: GetResponse = res.json::<GetResponse>().await?;

    println!("Response: {:?}", res_json);
    println!("Response: {:#?}", res_json);

    println!("\nYour IP: {}", res_json.origin);
    println!("URL: {}", res_json.url);


    println!("Args lang: {:?}", res_json.args.get("lang"));
    println!("Args count: {}", res_json.args.len());

    println!("Header accept: {}", res_json.headers.accept);
    println!("Header host: {}", res_json.headers.host);
    println!("Header xAmznTraceId: {}", res_json.headers.x_amzn_trace_id);

    Ok(())
}