use axum::{
    Router,
    extract::{Path, Query, Request},
    http::{HeaderMap, HeaderName, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{any, get},
};
use hyper_tls::HttpsConnector;
use hyper_util::{
    client::legacy::{Client, connect::HttpConnector},
    rt::TokioExecutor,
};
use std::{collections::HashMap, str::FromStr};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};

const PROXY_TARGET: &str = "https://httpbin.org";

type HttpClient = Client<HttpsConnector<HttpConnector>, axum::body::Body>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("proxy_handler=debug,tower_http=debug")
        .init();

    let https = HttpsConnector::new();
    let client: HttpClient = Client::builder(TokioExecutor::new()).build(https);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/proxy/{*path}", any(proxy_handler))
        .fallback(fallback_proxy)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(client);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    info!("🚀 Proxy server starting on http://0.0.0.0:3000");
    info!("📡 Proxying requests to: {}", PROXY_TARGET);
    info!("🔗 Health check available at: http://localhost:3000/health");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn health_check() -> impl IntoResponse {
    let response = serde_json::json!({
        "status": "healthy",
        "proxy_target": PROXY_TARGET,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    axum::Json(response)
}

async fn proxy_handler(
    axum::extract::State(client): axum::extract::State<HttpClient>,
    Path(path): Path<String>,
    Query(query_params): Query<HashMap<String, String>>,
    method: Method,
    headers: HeaderMap,
    request: Request,
) -> Result<Response, StatusCode> {
    info!("Proxying {} request to path: /{}", method, path);

    let mut target_url = format!("{}/{}", PROXY_TARGET, path);

    if !query_params.is_empty() {
        let query_string: Vec<String> = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        target_url.push('?');
        target_url.push_str(&query_string.join("&"));
    }

    proxy_request(client, target_url, method, headers, request).await
}

async fn fallback_proxy(
    axum::extract::State(client): axum::extract::State<HttpClient>,
    uri: Uri,
    method: Method,
    headers: HeaderMap,
    request: Request,
) -> Result<Response, StatusCode> {
    let path = uri.path();
    let query = uri.query().unwrap_or("");

    info!("Fallback proxy for {} request to: {}", method, path);

    let mut target_url = format!("{}{}", PROXY_TARGET, path);
    if !query.is_empty() {
        target_url.push('?');
        target_url.push_str(query);
    }

    proxy_request(client, target_url, method, headers, request).await
}

async fn proxy_request(
    client: HttpClient,
    target_url: String,
    method: Method,
    mut headers: HeaderMap,
    request: Request,
) -> Result<Response, StatusCode> {
    let uri = match target_url.parse::<Uri>() {
        Ok(uri) => uri,
        Err(e) => {
            error!("Failed to parse target URL {}: {}", target_url, e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    remove_hop_by_hop_headers(&mut headers);

    let (_parts, body) = request.into_parts();

    let mut proxy_request = hyper::Request::builder().method(method).uri(uri);

    for (name, value) in headers.iter() {
        proxy_request = proxy_request.header(name, value);
    }

    let proxy_request = match proxy_request.body(body) {
        Ok(req) => req,
        Err(e) => {
            error!("Failed to build proxy request: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    info!("Sending request to: {}", target_url);
    let response = match client.request(proxy_request).await {
        Ok(response) => response,
        Err(e) => {
            error!("Proxy request failed for URL {}: {:?}", target_url, e);
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    let (parts, body) = response.into_parts();
    let mut response_headers = parts.headers;

    remove_hop_by_hop_headers(&mut response_headers);

    let mut response_builder = Response::builder().status(parts.status);

    for (name, value) in response_headers.iter() {
        response_builder = response_builder.header(name, value);
    }

    match response_builder.body(axum::body::Body::new(body)) {
        Ok(response) => Ok(response),
        Err(e) => {
            error!("Failed to build response: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn remove_hop_by_hop_headers(headers: &mut HeaderMap) {
    let hop_by_hop_headers = [
        "connection",
        "keep-alive",
        "proxy-authenticate",
        "proxy-authorization",
        "te",
        "trailers",
        "transfer-encoding",
        "upgrade",
    ];

    for header in &hop_by_hop_headers {
        headers.remove(*header);
    }

    if let Some(connection_header) = headers.get("connection") {
        if let Ok(connection_str) = connection_header.to_str() {
            let headers_to_remove: Vec<HeaderName> = connection_str
                .split(',')
                .filter_map(|name| {
                    let name = name.trim();
                    HeaderName::from_str(name).ok()
                })
                .collect();

            for header_name in headers_to_remove {
                headers.remove(&header_name);
            }
        }
    }
}
