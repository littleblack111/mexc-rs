use crate::futures::ws::{
    acquire_websocket::{AcquireWebsocketForTopicsError, AcquireWebsocketsForTopics, AcquireWebsocketsForTopicsParams},
    auth::FuturesWebsocketAuth,
    topic::FuturesTopic,
    MexcFuturesWebsocketClient, SendableMessage,
};
use async_channel::SendError;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug)]
pub struct FuturesSubscribeParams {
    pub auth: Option<FuturesWebsocketAuth>,
    pub topics: Vec<FuturesTopic>,
}

impl Default for FuturesSubscribeParams {
    fn default() -> Self {
        Self::new(
            None,
            Vec::new(),
        )
    }
}

impl FuturesSubscribeParams {
    pub fn new(auth: Option<FuturesWebsocketAuth>, topics: Vec<FuturesTopic>) -> Self {
        Self {
            auth,
            topics,
        }
    }

    pub fn with_auth(mut self, auth: FuturesWebsocketAuth) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn with_topic(mut self, topic: FuturesTopic) -> Self {
        self.topics
            .push(topic);
        self
    }

    pub fn with_topics(mut self, topics: Vec<FuturesTopic>) -> Self {
        self.topics
            .extend(topics);
        self
    }
}

#[derive(Debug, Clone)]
pub struct FuturesSubscribeOutput {}

#[derive(Debug, thiserror::Error)]
pub enum FuturesSubscribeError {
    #[error("Tungestenite error: {0}")]
    TungesteniteError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Failed to send message through channel: {0}")]
    SendError(#[from] SendError<SendableMessage>),

    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

#[async_trait]
pub trait FuturesSubscribe {
    async fn subscribe(self: Arc<Self>, params: FuturesSubscribeParams) -> Result<FuturesSubscribeOutput, FuturesSubscribeError>;
}

#[async_trait]
impl FuturesSubscribe for MexcFuturesWebsocketClient {
    async fn subscribe(self: Arc<Self>, params: FuturesSubscribeParams) -> Result<FuturesSubscribeOutput, FuturesSubscribeError> {
        let mut acquire_websocket_params = AcquireWebsocketsForTopicsParams::for_topics(params.topics);
        if let Some(auth) = params.auth {
            acquire_websocket_params = acquire_websocket_params.with_auth(auth);
        }
        let acquire_output = match self
            .clone()
            .acquire_websockets_for_topics(acquire_websocket_params)
            .await
        {
            Ok(x) => x,
            Err(err) => match err {
                AcquireWebsocketForTopicsError::TungesteniteError(err) => {
                    return Err(FuturesSubscribeError::TungesteniteError(err));
                }
                AcquireWebsocketForTopicsError::SendError(err) => {
                    return Err(FuturesSubscribeError::SendError(err));
                }
                AcquireWebsocketForTopicsError::SerdeError(err) => {
                    return Err(FuturesSubscribeError::SerdeError(err));
                }
            },
        };

        for acquired_ws in acquire_output
            .websockets
            .into_iter()
        {
            // Update the topics in the websocket entry
            let mut topics = acquired_ws
                .websocket_entry
                .topics
                .write()
                .await;
            let topics_websocket_entry_does_not_have = acquired_ws
                .for_topics
                .into_iter()
                .filter(|topic| !topics.contains(topic))
                .collect::<Vec<FuturesTopic>>();
            topics.extend(topics_websocket_entry_does_not_have);
        }

        Ok(FuturesSubscribeOutput {})
    }
}
