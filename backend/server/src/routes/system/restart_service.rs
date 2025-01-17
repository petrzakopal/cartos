use std::{process::{Command, ExitStatus}, sync::Arc};

use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, HeaderMap, Response},
    response::IntoResponse,
    Json,
};
use common::{types::database::LogEntry, utils::perform_reset_with_usb_unplug};
use futures::TryStreamExt;
use serde_json::Value;
use sqlx::Row;
use tracing::{debug, error, info, warn};

use crate::create_routes::AppState;

pub async fn do_restart_service(
    headers: HeaderMap,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<Value>,
) -> impl IntoResponse {

        let res: Value = serde_json::from_str(
            r#"{"message":"Restarting the app.", "status":"error"}"#,
        )
        .unwrap();

    let response_builder = Response::builder()
        .status(500)
        .header(CONTENT_TYPE, "application/json")
        .body(Json(res).into_response().into_body())
        .unwrap();

    perform_reset_with_usb_unplug::perform_reset_with_nfc_usb_unplug().await;
    //let service_name = "cartos-backend.service"; // Replace with your actual service name
    //let status: ExitStatus = Command::new("systemctl")
    //    .arg("restart")
    //    .arg(service_name)
    //    .status()
    //    .expect("Failed to execute command");

    return response_builder;
}
