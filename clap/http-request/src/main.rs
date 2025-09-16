use clap::{Parser, ValueEnum};
use reqwest::{header::HeaderMap, Client, Method, Response, redirect::Policy};
use serde_json::Value;
use std::time::Duration;

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

impl From<HttpMethod> for Method {
    fn from(m: HttpMethod) -> Self {
        match m {
            HttpMethod::Get => Method::GET,
            HttpMethod::Post => Method::POST,
            HttpMethod::Put => Method::PUT,
            HttpMethod::Delete => Method::DELETE,
            HttpMethod::Patch => Method::PATCH,
            HttpMethod::Head => Method::HEAD,
            HttpMethod::Options => Method::OPTIONS,
        }
    }
}

#[derive(Parser, Debug)]
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

    /// Add a header (format: 'Key: Value'), can be repeated
    #[arg(short = 'H', long = "header")]
    header: Vec<String>,

    /// Raw request body (ignored if --json is provided)
    #[arg(short = 'd', long = "data")]
    body: Option<String>,

    /// JSON request body (sets Content-Type to application/json)
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

fn parse_headers(items: &[String]) -> Result<HeaderMap, String> {
    let mut map = HeaderMap::new();
    for h in items {
        // Split at the first ':'
        let (k, v) = h.split_once(':').ok_or_else(|| {
            format!("Invalid header format (expected 'Key: Value'): {}", h)
        })?;
        let key = k.trim();
        let val = v.trim();

        let name = reqwest::header::HeaderName::from_bytes(key.as_bytes())
            .map_err(|e| format!("Invalid header name '{}': {}", key, e))?;
        let value = reqwest::header::HeaderValue::from_str(val)
            .map_err(|e| format!("Invalid header value for '{}': {}", key, e))?;

        map.append(name, value);
    }
    Ok(map)
}

async fn print_response(resp: Response, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        eprintln!("> HTTP/{:?} {}", resp.version(), resp.status());
        for (k, v) in resp.headers() {
            eprintln!("> {}: {}", k, v.to_str().unwrap_or("<binary>"));
        }
        eprintln!();
    }

    let status = resp.status();
    let text = resp.text().await?;

    // Try to pretty-print JSON; otherwise print raw
    if let Ok(json) = serde_json::from_str::<Value>(&text) {
        println!("{}", serde_json::to_string_pretty(&json)?);
    } else {
        println!("{}", text);
    }

    if !status.is_success() && verbose {
        eprintln!("Note: non-success status {}", status);
    }

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // Build client
    let client = Client::builder()
        .timeout(Duration::from_secs(args.timeout))
        .redirect(if args.no_redirect {
            Policy::none()
        } else {
            // Otherwise, the client will follow redirects up to 10 times
            // Preventing infinite redirect loops.
            Policy::limited(10)
        })
        .build()?;

    // Method & URL
    let method: Method = args.method.clone().into();
    let url = &args.url;

    // Headers
    let mut headers = parse_headers(&args.header)
        .map_err(|e| format!("Header error: {}", e))?;

    println!("Parsed headers: {:#?}", headers);

    // Body selection
    let mut body_to_send: Option<Vec<u8>> = None;

    if let Some(json_str) = args.json.as_ref() {
        // Validate JSON once; also set Content-Type if not provided
        let _parsed: Value = serde_json::from_str(json_str)
            .map_err(|e| format!("Provided --json is not valid JSON: {}", e))?;

        println!("Sending JSON body: {}", json_str);
        if !headers.contains_key(reqwest::header::CONTENT_TYPE) {
            headers.insert(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/json"),
            );
        }
        body_to_send = Some(json_str.as_bytes().to_vec());
    } else if let Some(raw) = args.body.as_ref() {
        body_to_send = Some(raw.as_bytes().to_vec());
    }

    if args.verbose {
        eprintln!("> {} {}", method, url);
        for (k, v) in headers.iter() {
            eprintln!("> {}: {}", k, v.to_str().unwrap_or("<binary>"));
        }
        if let Some(ref b) = body_to_send {
            // Print a preview only
            let preview = String::from_utf8_lossy(b);
            let preview = if preview.len() > 500 {
                format!("{}... ({} bytes)", &preview[..500], b.len())
            } else {
                preview.to_string()
            };
            eprintln!(">\n{}", preview);
        }
        eprintln!();
    }

    // Build request
    let mut req = client.request(method, url).headers(headers);

    if let Some(b) = body_to_send {
        req = req.body(b);
    }

    // Send and print
    let resp = req.send().await?;
    print_response(resp, args.verbose).await?;

    Ok(())
}
