use crate::futures::ws::{auth::FuturesWebsocketAuth, endpoint::MexcFuturesWebsocketEndpoint, topic::FuturesTopic};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod acquire_websocket;
pub mod auth;
pub mod endpoint;
pub mod message;
pub mod stream;
pub mod subscribe;
pub mod topic;
pub mod unsubscribe;

#[derive(Debug, Clone)]
pub struct FuturesWebsocketEntry {
    pub id: Uuid,
    pub auth: Option<FuturesWebsocketAuth>,
    pub topics: Arc<RwLock<Vec<FuturesTopic>>>,
    pub message_tx: Arc<RwLock<async_channel::Sender<SendableMessage>>>,
}

#[derive(Debug)]
struct Inner {
    pub websockets: Vec<Arc<FuturesWebsocketEntry>>,
}

#[derive(Debug, Clone)]
pub struct MexcFuturesWebsocketClient {
    inner: Arc<RwLock<Inner>>,
    ws_endpoint: Arc<MexcFuturesWebsocketEndpoint>,
    broadcast_tx: tokio::sync::broadcast::Sender<Arc<message::FuturesMessage>>,
}

impl MexcFuturesWebsocketClient {
    pub fn new_with_endpoint(ws_endpoint: MexcFuturesWebsocketEndpoint) -> Self {
        let (broadcast_tx, _broadcast_rx) = tokio::sync::broadcast::channel(1024);

        Self {
            inner: Arc::new(
                RwLock::new(
                    Inner {
                        websockets: Vec::new(),
                    },
                ),
            ),
            ws_endpoint: Arc::new(ws_endpoint),
            broadcast_tx,
        }
    }

    pub fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

impl Default for MexcFuturesWebsocketClient {
    fn default() -> Self {
        Self::new_with_endpoint(MexcFuturesWebsocketEndpoint::Base)
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum SendableMessage {
    Subscription(serde_json::Value),
    Unsubscription(serde_json::Value),
    Ping,
    Login(serde_json::Value),
}
