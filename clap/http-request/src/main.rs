use clap::{Parser, ValueEnum};
use reqwest::{Client, Method, Response};
use serde_json::{Value, from_str};
use std::collections::HashMap;
use std::time::Duration;
use tokio;


#[derive(Debug, Clone, ValueEnum)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

#[derive(Parser)]
#[command(name = "http-client")]
#[command(version = "1.0.0")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(about = "A simple HTTP client CLI tool")]
struct Cli {
    /// The URL to request
    url: String,

    /// HTTP method to use
    #[arg(short = 'X', long = "method", default_value = "get")]
    method: HttpMethod,

    /// Add a header (format: 'Key: Value')
    #[arg(short = 'H', long = "header")]
    header: Vec<String>,

    /// Request body data
    #[arg(short = 'd', long = "data")]
    body: Option<String>,

    /// Send JSON data (sets Content-Type header)
    #[arg(short = 'j', long = "json")]
    json: Option<String>,

    /// Request timeout in seconds
    #[arg(short = 't', long = "timeout", default_value = "30")]
    timeout: u64,

    /// Don't follow redirects
    #[arg(long = "no-redirect")]
    no_redirect: bool,

    /// Verbose output
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

#[derive(Debug)]
struct HttpRequest {
    method: Method
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let request = HttpResponse::new(&cli)?;

    Ok(())
}
