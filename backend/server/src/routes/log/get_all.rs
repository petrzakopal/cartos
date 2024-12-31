use std::sync::Arc;

use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, HeaderMap, Response},
    response::IntoResponse,
    Json,
};
use futures::TryStreamExt;
use serde_json::Value;
use sqlx::Row;
use tracing::{debug, error, warn};

use crate::create_routes::AppState;

pub async fn get_all_logs(
    headers: HeaderMap,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    let test_json: Value = serde_json::from_str(r#"{"key":"value", "key1":"value1"}"#).unwrap();
    if app_state.db_sqlite_pool.is_closed() {
        error!("The database connection pool is closed.");
    } else {
        debug!("Database connection pool is open.");
    }

    let mut query_result = sqlx::query(r#"SELECT * from log;"#).fetch(&app_state.db_sqlite_pool);

    while let Ok(Some(res)) = query_result.try_next().await {
        let email: Option<String> = match res.try_get("cardSerialNumber") {
            Ok(Some(data)) => {
                debug!("Obtained cardSerialNumber from logs {}", data);
                Some(data)
            }
            Ok(None) => {
                warn!("Did not find any logs");
                None
            }
            Err(e) => {
                error!("Could not obtain the data from logs {:#?}", e);
                None
            }
        };
    }

    let response_builder = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "application/json")
        .body(Json(test_json).into_response().into_body())
        .unwrap();
    return response_builder;
}
