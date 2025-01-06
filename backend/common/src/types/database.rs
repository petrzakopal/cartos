use serde::Serialize;

#[derive(Serialize, sqlx::FromRow, Debug, Clone)]
pub struct LogEntry {
    pub id: i32,
    pub timestamp: String,
    //#[serde(rename = "cardSerialNumber")]
    //#[sqlx(rename = "cardSerialNumber")]
    pub card_serial_number: String,
    pub email: String,
    pub status: String,
    pub note: Option<String>,
}

#[derive(Serialize, sqlx::FromRow, Debug, Clone)]
pub struct UserEntry {
    pub id: i32,
    pub card_serial_number: String,
    pub email: String,
    pub note: Option<String>,
    pub updated_at: String,
    pub status: String,
}

#[derive(Serialize, sqlx::FromRow, Debug, Clone)]
pub struct CardRead {
    pub card_serial_number: String,
}

#[derive(Serialize, sqlx::FromRow, Debug, Clone)]
pub struct ApplicationState {
    pub id: i32,
    pub do_reset: i32,
}
