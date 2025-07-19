use serde_json::json;
use crate::http::{HttpRequest, HttpResponse};
use crate::state::AppState;

pub async fn get_all_users(state: &AppState) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let users = state.get_all_users().await;

    let response_data = json!({
        "users": users,
        "total": users.len(),
    });

    HttpResponse::with_json(200, &response_data).map_err(Into::into)
}