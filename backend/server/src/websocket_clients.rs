use std::{collections::HashMap, sync::Arc, collections::hash_map::Entry};
use axum::extract::ws::{Message, WebSocket};
use futures::{lock::Mutex, sink::SinkExt, stream::SplitSink};
use tracing::error;
use uuid::Uuid;

// New type to store client senders
type ClientSender = Arc<Mutex<SplitSink<WebSocket, Message>>>;

#[derive(Clone)]
pub struct Clients {
    inner: Arc<Mutex<HashMap<Uuid, ClientSender>>>,
}

impl Clients {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add(&self, sender: ClientSender) -> Uuid {
        let id = Uuid::new_v4();
        self.inner.lock().await.insert(id, sender);
        id
    }

    pub async fn remove(&self, id: &Uuid) {
        self.inner.lock().await.remove(id);
    }

    pub async fn broadcast(&self, message: Message) {
        let mut clients = self.inner.lock().await;
        let mut dead_clients = Vec::new();

        for (id, client) in clients.iter() {
            if let Err(_) = client.lock().await.send(message.clone()).await {
                dead_clients.push(*id);
            }
        }

        // Remove dead clients
        for id in dead_clients {
            clients.remove(&id);
        }
    }

    pub async fn len(&self) -> usize {
        self.inner.lock().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.inner.lock().await.is_empty()
    }
}

impl Default for Clients {
    fn default() -> Self {
        Self::new()
    }
}
