use std::sync::Arc;

use axum::{extract::State, http::{header::CONTENT_TYPE, HeaderMap, Response}, response::IntoResponse, Json};
use serde_json::Value;

use crate::create_routes::AppState;

pub async fn get_all_logs(
    headers: HeaderMap,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<Value>,
) -> impl IntoResponse {

    let test_json : Value = serde_json::from_str(r#"{"key":"value", "key1":"value1"}"#).unwrap();

 let response_builder = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "application/json")
        .body(Json(test_json).into_response().into_body())
        .unwrap();
    return response_builder;
}
