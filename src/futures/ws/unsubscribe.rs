use crate::futures::ws::{topic::FuturesTopic, MexcFuturesWebsocketClient, SendableMessage};
use async_channel::SendError;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug)]
pub struct FuturesUnsubscribeParams {
    pub topics: Vec<FuturesTopic>,
}

impl Default for FuturesUnsubscribeParams {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl FuturesUnsubscribeParams {
    pub fn new(topics: Vec<FuturesTopic>) -> Self {
        Self {
            topics,
        }
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
pub struct FuturesUnsubscribeOutput {}

#[derive(Debug, thiserror::Error)]
pub enum FuturesUnsubscribeError {
    #[error("Failed to send message through channel: {0}")]
    SendError(#[from] SendError<SendableMessage>),
}

#[async_trait]
pub trait FuturesUnsubscribe {
    async fn unsubscribe(self: Arc<Self>, params: FuturesUnsubscribeParams) -> Result<FuturesUnsubscribeOutput, FuturesUnsubscribeError>;
}

#[async_trait]
impl FuturesUnsubscribe for MexcFuturesWebsocketClient {
    async fn unsubscribe(self: Arc<Self>, params: FuturesUnsubscribeParams) -> Result<FuturesUnsubscribeOutput, FuturesUnsubscribeError> {
        // Placeholder: send unsubscription for each topic
        for topic in params.topics {
            let _message = SendableMessage::Unsubscription(topic.to_unsubscription_message());
            // Need to get the tx from websocket entry
        }

        Ok(FuturesUnsubscribeOutput {})
    }
}
