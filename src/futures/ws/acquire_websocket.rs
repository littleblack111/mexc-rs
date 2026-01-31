use crate::futures::ws::{auth::FuturesWebsocketAuth, topic::FuturesTopic, FuturesWebsocketEntry, MexcFuturesWebsocketClient, SendableMessage};
use async_channel::SendError;
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::{sync::RwLock, time::Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use uuid::Uuid;

#[derive(Default)]
pub struct AcquireWebsocketsForTopicsParams {
    pub topics: Vec<FuturesTopic>,
    pub auth: Option<FuturesWebsocketAuth>,
}

impl AcquireWebsocketsForTopicsParams {
    pub fn for_topics(topics: Vec<FuturesTopic>) -> Self {
        Self {
            topics,
            auth: None,
        }
    }

    pub fn with_auth(mut self, auth: FuturesWebsocketAuth) -> Self {
        self.auth = Some(auth);
        self
    }
}

#[derive(Debug)]
pub struct AcquireWebsocketsForTopicsOutput {
    pub websockets: Vec<AcquiredWebsocket>,
}

#[derive(Debug)]
pub struct AcquiredWebsocket {
    pub websocket_entry: Arc<FuturesWebsocketEntry>,
    pub for_topics: Vec<FuturesTopic>,
}

#[derive(Debug, thiserror::Error)]
pub enum AcquireWebsocketForTopicsError {
    #[error("Tungestenite error: {0}")]
    TungesteniteError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Failed to send message through channel: {0}")]
    SendError(#[from] SendError<SendableMessage>),

    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

#[async_trait]
pub trait AcquireWebsocketsForTopics {
    async fn acquire_websockets_for_topics(self: Arc<Self>, params: AcquireWebsocketsForTopicsParams) -> Result<AcquireWebsocketsForTopicsOutput, AcquireWebsocketForTopicsError>;
}

#[async_trait]
impl AcquireWebsocketsForTopics for MexcFuturesWebsocketClient {
    async fn acquire_websockets_for_topics(self: Arc<Self>, params: AcquireWebsocketsForTopicsParams) -> Result<AcquireWebsocketsForTopicsOutput, AcquireWebsocketForTopicsError> {
        let (tx, _rx) = async_channel::unbounded::<SendableMessage>();

        let websocket_entry = Arc::new(
            FuturesWebsocketEntry {
                id: Uuid::new_v4(),
                auth: params
                    .auth
                    .clone(),
                topics: Arc::new(RwLock::new(Vec::new())),
                message_tx: Arc::new(RwLock::new(tx)),
            },
        );

        let ws_url = self
            .ws_endpoint
            .as_ref();
        let (ws_stream, _) = connect_async(ws_url.as_ref()).await?;
        let (write, read) = ws_stream.split();

        let _websocket_entry_clone = websocket_entry.clone();
        let broadcast_tx_clone = self
            .broadcast_tx
            .clone();

        tokio::spawn(
            async move {
                let mut read = read;
                while let Some(message) = read
                    .next()
                    .await
                {
                    match message {
                        Ok(Message::Text(text)) => {
                            if let Ok(raw_message) = serde_json::from_str::<crate::futures::ws::message::RawFuturesMessage>(&text) {
                                if let Ok(futures_message) = raw_message.try_into() {
                                    let _ = broadcast_tx_clone.send(Arc::new(futures_message));
                                }
                            }
                        }
                        Ok(Message::Ping(_)) => {
                            // Handle ping
                        }
                        Ok(Message::Pong(_)) => {
                            // Handle pong
                        }
                        Ok(Message::Close(_)) => break,
                        _ => {}
                    }
                }
            },
        );

        let write = Arc::new(RwLock::new(write));

        // Send login if auth
        if let Some(auth) = &params.auth {
            let req_time = chrono::Utc::now()
                .timestamp_millis()
                .to_string();
            let signature = auth.generate_signature(&req_time);
            let login_message = serde_json::json!({
                "method": "login",
                "param": {
                    "apiKey": auth.api_key,
                    "signature": signature,
                    "reqTime": req_time
                }
            });
            let sendable = SendableMessage::Login(login_message);
            let json = serde_json::to_string(&sendable).unwrap();
            let mut write_guard = write
                .write()
                .await;
            write_guard
                .send(Message::Text(json))
                .await?;
            drop(write_guard);
        }

        // Send ping periodically
        let write_ping = write.clone();
        tokio::spawn(
            async move {
                let mut interval = tokio::time::interval(Duration::from_secs(20));
                loop {
                    interval
                        .tick()
                        .await;
                    let ping_message = SendableMessage::Ping;
                    let json = serde_json::to_string(&ping_message).unwrap();
                    if let Ok(mut write_guard) = write_ping.try_write() {
                        let _ = write_guard
                            .send(Message::Text(json))
                            .await;
                    }
                }
            },
        );

        // Send subscriptions
        for topic in &params.topics {
            let subscription = topic.to_subscription_message();
            let sendable = SendableMessage::Subscription(subscription);
            let json = serde_json::to_string(&sendable).unwrap();
            let mut write_guard = write
                .write()
                .await;
            write_guard
                .send(Message::Text(json))
                .await?;
            drop(write_guard);
        }

        let mut inner = self
            .inner
            .write()
            .await;
        inner
            .websockets
            .push(websocket_entry.clone());

        Ok(
            AcquireWebsocketsForTopicsOutput {
                websockets: vec![
                    AcquiredWebsocket {
                        websocket_entry,
                        for_topics: params.topics,
                    },
                ],
            },
        )
    }
}
