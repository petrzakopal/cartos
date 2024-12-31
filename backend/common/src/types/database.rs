use serde::Serialize;

#[derive(Serialize, sqlx::FromRow, Debug, Clone)]
pub struct LogEntry {
    id: i32,
    timestamp: String,
    //#[serde(rename = "cardSerialNumber")]
    #[sqlx(rename = "cardSerialNumber")]
    card_serial_number: String,
    result: String,
    note: Option<String>,
}
