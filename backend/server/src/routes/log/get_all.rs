use std::sync::Arc;

use axum::{extract::State, http::HeaderMap, response::IntoResponse, Json};
use serde_json::Value;

use crate::create_routes::AppState;

pub async fn get_all_logs(
    headers: HeaderMap,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
}
