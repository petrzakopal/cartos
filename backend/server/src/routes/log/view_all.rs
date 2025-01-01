use std::sync::Arc;

use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, HeaderMap, Response},
    response::IntoResponse,
    Json,
};
use common::types::database::LogEntry;
use futures::TryStreamExt;
use serde_json::Value;
use sqlx::Row;
use tracing::{debug, error, info, warn};

use crate::create_routes::AppState;

pub async fn view_all_logs(
    headers: HeaderMap,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    if app_state.db_sqlite_pool.is_closed() {
        error!("The database connection pool is closed.");
        let res: Value = serde_json::from_str(
            r#"{"message":"The database connection pool is closed.", "status":"error"}"#,
        )
        .unwrap();

        let response_builder = Response::builder()
            .status(500)
            .header(CONTENT_TYPE, "application/json")
            .body(Json(res).into_response().into_body())
            .unwrap();
        return response_builder;
    } else {
        debug!("Database connection pool is open.");
    }

    // Getting the data and mapping the results on a structn with correct naming
    let mut query_mapped: Vec<LogEntry> = sqlx::query_as(r#"SELECT * from log;"#)
        .fetch_all(&app_state.db_sqlite_pool)
        .await
        .unwrap();

    // How to loop through entries obtained from the query
    for entry in &query_mapped {
        debug!("Obtained entry from the db {:#?}", entry);
    }

    // How to get just one field from the query
    //let mut query_result = sqlx::query(r#"SELECT * from log;"#).fetch(&app_state.db_sqlite_pool);
    //while let Ok(Some(res)) = query_result.try_next().await {
    //    let serial_card_number: Option<String> = match res.try_get("cardSerialNumber") {
    //        Ok(Some(data)) => {
    //            debug!("Obtained cardSerialNumber from logs {}", data);
    //            Some(data)
    //        }
    //        Ok(None) => {
    //            warn!("Did not find any logs");
    //            None
    //        }
    //        Err(e) => {
    //            error!("Could not obtain the data from logs {:#?}", e);
    //            None
    //        }
    //    };
    //}

    let response_builder = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "application/json")
        .body(Json(query_mapped).into_response().into_body())
        .unwrap();
    return response_builder;
}
