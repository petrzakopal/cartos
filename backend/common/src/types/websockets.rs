use serde::Serialize;

use super::database::{CardRead, LogEntry, UserEntry};

#[derive(Serialize, Debug, Clone)]
pub enum MessageAction {
    CardRead,
    NewLogEntry,
    NewUserEntry,
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum WebsocketMessageData {
    UserEntry(UserEntry),
    LogEntry(LogEntry),
    CardRead(CardRead)
}

#[derive(Serialize, sqlx::FromRow, Debug, Clone)]
pub struct WebsocketMessageBody {
    pub action: MessageAction,
    pub data: WebsocketMessageData,
}
