use dotenv::dotenv;
use futures::StreamExt;
use mexc_rs::futures::ws::{
    stream::FuturesStream,
    subscribe::{FuturesSubscribe, FuturesSubscribeParams},
    topic::FuturesTopic,
    MexcFuturesWebsocketClient,
};

#[tokio::main]
async fn main() {
    std::env::set_var(
        "RUST_LOG",
        "mexc_rs=debug,futures_simple_public_subscription=trace",
    );
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let ws_client = MexcFuturesWebsocketClient::default().into_arc();
    ws_client
        .clone()
        .subscribe(FuturesSubscribeParams::default().with_topics(vec![FuturesTopic::Tickers]))
        .await
        .expect("Failed to subscribe");

    let mut stream = ws_client.stream();
    let mut count = 0;
    while let Some(message) = stream
        .next()
        .await
    {
        dbg!(&message);
        count += 1;
        if count >= 5 {
            break;
        }
    }
}
