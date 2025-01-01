use std::sync::Arc;

use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, HeaderMap, Response},
    response::IntoResponse,
    Json,
};
use common::{
    types::{channels::CardData, database::UserEntry},
};
use futures::TryStreamExt;
use serde_json::{json, Value};
use sqlx::Row;
use tracing::{debug, error, info, warn};

use crate::create_routes::AppState;

pub async fn add_user(
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

    let mut card_data: CardData = CardData::new();

    // Obtain the email to query from the body to database
    card_data.serial_number_string = body
        .get("card_data")
        .and_then(|card_data| card_data.get("serial_number"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    card_data.email = body
        .get("email")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    card_data.note = body
        .get("note")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Check if the user submission is valid

    match card_data.validate() {
        Ok(_d) => {
            info!("Submitted user data is valid, proceeding to add user to the database.");
        }
        Err(errors) => {
            error!(
                "Submitted user data is not valid. Missing fields. Will not add to the database.\n Message: {:#?}", errors
            );

            let res: Value = json!({
                "message": format!("Submitted user data is not valid. Missing fields."),
                "additional_info": errors,
                "status": "error"
            });

            let response_builder = Response::builder()
                .status(200)
                .header(CONTENT_TYPE, "application/json")
                .body(Json(res).into_response().into_body())
                .unwrap();
            return response_builder;
        }
    }

    // Getting the data and mapping the results on a structn with correct naming
    let mut query =
        sqlx::query(r#"INSERT INTO user (card_serial_number, email, note) VALUES (?, ?, ?);"#)
            .bind(&card_data.serial_number_string)
            .bind(&card_data.email)
            .bind(&card_data.note)
            .execute(&app_state.db_sqlite_pool)
            .await
            .unwrap();


    let res: Value = json!({
        "message": format!("Successfully added new user."),
        "email": &card_data.email,
        "card_data": {
        "serial_number" : &card_data.serial_number_string
    },
        "note": &card_data.note,
        "status": "success"
    });

    let response_builder = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "application/json")
        .body(Json(res).into_response().into_body())
        .unwrap();
    return response_builder;
}