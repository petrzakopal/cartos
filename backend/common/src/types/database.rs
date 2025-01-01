use serde::Serialize;

#[derive(Serialize, sqlx::FromRow, Debug, Clone)]
pub struct LogEntry {
    id: i32,
    timestamp: String,
    //#[serde(rename = "cardSerialNumber")]
    //#[sqlx(rename = "cardSerialNumber")]
    card_serial_number: String,
    email: String,
    result: String,
    note: Option<String>,
}

#[derive(Serialize, sqlx::FromRow, Debug, Clone)]
pub struct UserEntry {
    id: i32,
    card_serial_number: String,
    email: String,
    note: Option<String>,
}