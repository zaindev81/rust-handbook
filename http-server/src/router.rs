use crate::http::{HttpRequest, HttpResponse};
use crate::state::AppState;
use crate::handlers;

pub struct Router {
    state: AppState,
}

impl Router {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn route(&self, request: &HttpRequest) -> Result<HttpResponse, Box<dyn std::error::Error>> {
        println!("🔍 Routing: {} {}", request.method, request.path);

        match (request.method.as_str(), request.path.as_str()) {
            // User endpoints
            ("GET", "/api/users") => {
                handlers::users::get_all_users(&self.state).await
            },
            _ => {
                HttpResponse::error(404, "Endpoint not found").map_err(Into::into)
            }
        }
    }
}