use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardData {
    pub serial_number_string: String,
}

#[derive(Debug)]
pub struct CardDataBroadcastChannel  {
    pub tx: tokio::sync::broadcast::Sender<CardData>,
    pub rx: tokio::sync::broadcast::Receiver<CardData>
}
