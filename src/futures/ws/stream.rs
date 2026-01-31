use crate::futures::ws::{message::FuturesMessage, MexcFuturesWebsocketClient};
use futures::{stream::BoxStream, StreamExt};
use std::sync::Arc;

pub trait FuturesStream {
    fn stream<'a>(self: Arc<Self>) -> BoxStream<'a, Arc<FuturesMessage>>;
}

impl FuturesStream for MexcFuturesWebsocketClient {
    fn stream<'a>(self: Arc<Self>) -> BoxStream<'a, Arc<FuturesMessage>> {
        let mut rx = self
            .broadcast_tx
            .subscribe();
        let stream = async_stream::stream! {
            while let Ok(message) = rx.recv().await {
                yield message;
            }
        };
        stream.boxed()
    }
}
