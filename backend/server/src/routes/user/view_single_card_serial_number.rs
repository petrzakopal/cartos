use std::sync::Arc;

use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, HeaderMap, Response},
    response::IntoResponse,
    Json,
};
use common::types::database::UserEntry;
use futures::TryStreamExt;
use serde_json::{json, Value};
use sqlx::Row;
use tracing::{debug, error, info, warn};

use crate::create_routes::AppState;

pub async fn view_single_user_by_card_serial_number(
    headers: HeaderMap,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<Value>,
) -> impl IntoResponse {

    // Check if the db pool is available
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

    // Obtain the email to query from the body to database
    let field_to_query = body.get("card_data").and_then(|card_data| card_data.get("serial_number")).and_then(|v| v.as_str()).unwrap_or("");

    // If no email to search for has been specified
    if field_to_query == "" {
        let res: Value = serde_json::from_str(
            r#"{"message":"No field in the request body was specified.", "status":"error"}"#,
        )
        .unwrap();
        let response_builder = Response::builder()
            .status(501)
            .header(CONTENT_TYPE, "application/json")
            .body(Json(res).into_response().into_body())
            .unwrap();
        return response_builder;

    }

    debug!("email to search for in the db: {}", field_to_query);

    // Getting the data and mapping the results on a structn with correct naming
    let mut query_mapped: Vec<UserEntry> = sqlx::query_as(r#"SELECT * from user WHERE card_serial_number = ?;"#)
        .bind(field_to_query)
        .fetch_all(&app_state.db_sqlite_pool)
        .await
        .unwrap();

    // How to loop through entries obtained from the query
    for entry in &query_mapped {
        debug!("Obtained entry from the db {:#?}", entry);
    }

    // If email has been specified but the results were not found
    if query_mapped.len() == 0 {
        let res: Value = json!({
        "message": format!("No entries with provided field_to_query: {} were found.", field_to_query),
        "status": "result_not_found"
    });

        let response_builder = Response::builder()
            .status(501)
            .header(CONTENT_TYPE, "application/json")
            .body(Json(res).into_response().into_body())
            .unwrap();
        return response_builder;


    }

    let response_builder = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "application/json")
        .body(Json(query_mapped).into_response().into_body())
        .unwrap();
    return response_builder;
}
