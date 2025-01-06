use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::websockets::WebsocketMessageBody;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardData {
    pub serial_number_string: String,
    pub email: String,
    pub note: String,
    pub status: String,
}

impl CardData {
    pub fn new() -> Self {
        Self {
            serial_number_string: String::default(),
            email: String::default(),
            note: String::default(),
            status: String::default(),
        }
    }

    pub fn validate(&self) -> Result<(), HashMap<String, String>> {
        let mut errors: HashMap<String, String> = HashMap::new();

        if self.serial_number_string.is_empty() {
            errors.insert(
                "card_data.serial_number".to_string(),
                "is_empty".to_string(),
            );
        }
        if self.email.is_empty() {
            errors.insert("email".to_string(), "is_empty".to_string());
        }
        if self.status.is_empty() {
            errors.insert("status".to_string(), "is_empty".to_string());
        }

        if errors.is_empty() {
            return Ok(());
        } else {
            return Err(errors);
        }
    }
}

#[derive(Debug)]
pub struct CardDataBroadcastChannel {
    pub tx: tokio::sync::broadcast::Sender<CardData>,
    pub rx: tokio::sync::broadcast::Receiver<CardData>,
}

#[derive(Debug)]
pub struct WebsocketBodyBroadcastChannel {
    pub tx: tokio::sync::broadcast::Sender<WebsocketMessageBody>,
    pub rx: tokio::sync::broadcast::Receiver<WebsocketMessageBody>,
}
