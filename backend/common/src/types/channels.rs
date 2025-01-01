use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardData {
    pub serial_number_string: String,
    pub email: String,
    pub note: String,
}

impl CardData {
    pub fn new() -> Self {
        Self {
            serial_number_string: String::default(),
            email: String::default(),
            note: String::default()
        }
    }

    pub fn validate(&self) -> Result<(), HashMap<String, String>> {
        let mut errors : HashMap<String, String> = HashMap::new();

        if self.serial_number_string.is_empty() {
            errors.insert("card_data.serial_number".to_string(), "is_empty".to_string());
        }
            if self.email.is_empty() {
            errors.insert("email".to_string(), "is_empty".to_string());
        }

        if errors.is_empty() {
            return Ok(());
        }
        else {
            return Err(errors);
        }

    }
}

#[derive(Debug)]
pub struct CardDataBroadcastChannel {
    pub tx: tokio::sync::broadcast::Sender<CardData>,
    pub rx: tokio::sync::broadcast::Receiver<CardData>,
}
