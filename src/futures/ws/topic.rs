#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum FuturesTopic {
    Tickers,
    Ticker(TickerTopic),
    Deal(DealTopic),
    Depth(DepthTopic),
    DepthStep(DepthStepTopic),
    Kline(KlineTopic),
    FundingRate(FundingRateTopic),
    IndexPrice(IndexPriceTopic),
    FairPrice(FairPriceTopic),
    Contract,
    EventContract,
}

impl FuturesTopic {
    pub fn requires_auth(&self) -> bool {
        match self {
            FuturesTopic::Tickers => false,
            FuturesTopic::Ticker(_) => false,
            FuturesTopic::Deal(_) => false,
            FuturesTopic::Depth(_) => false,
            FuturesTopic::DepthStep(_) => false,
            FuturesTopic::Kline(_) => false,
            FuturesTopic::FundingRate(_) => false,
            FuturesTopic::IndexPrice(_) => false,
            FuturesTopic::FairPrice(_) => false,
            FuturesTopic::Contract => false,
            FuturesTopic::EventContract => false,
        }
    }

    pub fn to_subscription_message(&self) -> serde_json::Value {
        match self {
            FuturesTopic::Tickers => serde_json::json!({
                "method": "sub.tickers",
                "param": {}
            }),
            FuturesTopic::Ticker(ticker) => serde_json::json!({
                "method": "sub.ticker",
                "param": {
                    "symbol": ticker.symbol
                }
            }),
            FuturesTopic::Deal(deal) => serde_json::json!({
                "method": "sub.deal",
                "param": {
                    "symbol": deal.symbol
                }
            }),
            FuturesTopic::Depth(depth) => serde_json::json!({
                "method": "sub.depth",
                "param": {
                    "symbol": depth.symbol
                }
            }),
            FuturesTopic::DepthStep(depth_step) => serde_json::json!({
                "method": "sub.depth.step",
                "param": {
                    "symbol": depth_step.symbol,
                    "step": depth_step.step
                }
            }),
            FuturesTopic::Kline(kline) => serde_json::json!({
                "method": "sub.kline",
                "param": {
                    "symbol": kline.symbol,
                    "interval": kline.interval
                }
            }),
            FuturesTopic::FundingRate(fr) => serde_json::json!({
                "method": "sub.funding.rate",
                "param": {
                    "symbol": fr.symbol
                }
            }),
            FuturesTopic::IndexPrice(ip) => serde_json::json!({
                "method": "sub.index.price",
                "param": {
                    "symbol": ip.symbol
                }
            }),
            FuturesTopic::FairPrice(fp) => serde_json::json!({
                "method": "sub.fair.price",
                "param": {
                    "symbol": fp.symbol
                }
            }),
            FuturesTopic::Contract => serde_json::json!({
                "method": "sub.contract"
            }),
            FuturesTopic::EventContract => serde_json::json!({
                "method": "sub.event.contract"
            }),
        }
    }

    pub fn to_unsubscription_message(&self) -> serde_json::Value {
        match self {
            FuturesTopic::Tickers => serde_json::json!({
                "method": "unsub.tickers",
                "param": {}
            }),
            FuturesTopic::Ticker(ticker) => serde_json::json!({
                "method": "unsub.ticker",
                "param": {
                    "symbol": ticker.symbol
                }
            }),
            FuturesTopic::Deal(deal) => serde_json::json!({
                "method": "unsub.deal",
                "param": {
                    "symbol": deal.symbol
                }
            }),
            FuturesTopic::Depth(depth) => serde_json::json!({
                "method": "unsub.depth",
                "param": {
                    "symbol": depth.symbol
                }
            }),
            FuturesTopic::DepthStep(depth_step) => serde_json::json!({
                "method": "unsub.depth.step",
                "param": {
                    "symbol": depth_step.symbol,
                    "step": depth_step.step
                }
            }),
            FuturesTopic::Kline(kline) => serde_json::json!({
                "method": "unsub.kline",
                "param": {
                    "symbol": kline.symbol
                }
            }),
            FuturesTopic::FundingRate(fr) => serde_json::json!({
                "method": "unsub.funding.rate",
                "param": {
                    "symbol": fr.symbol
                }
            }),
            FuturesTopic::IndexPrice(ip) => serde_json::json!({
                "method": "unsub.index.price",
                "param": {
                    "symbol": ip.symbol
                }
            }),
            FuturesTopic::FairPrice(fp) => serde_json::json!({
                "method": "unsub.fair.price",
                "param": {
                    "symbol": fp.symbol
                }
            }),
            FuturesTopic::Contract => serde_json::json!({
                "method": "unsub.contract"
            }),
            FuturesTopic::EventContract => serde_json::json!({
                "method": "unsub.event.contract"
            }),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TickerTopic {
    pub symbol: String,
}

impl TickerTopic {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DealTopic {
    pub symbol: String,
}

impl DealTopic {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DepthTopic {
    pub symbol: String,
}

impl DepthTopic {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DepthStepTopic {
    pub symbol: String,
    pub step: String,
}

impl DepthStepTopic {
    pub fn new(symbol: String, step: String) -> Self {
        Self {
            symbol,
            step,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct KlineTopic {
    pub symbol: String,
    pub interval: String,
}

impl KlineTopic {
    pub fn new(symbol: String, interval: String) -> Self {
        Self {
            symbol,
            interval,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FundingRateTopic {
    pub symbol: String,
}

impl FundingRateTopic {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IndexPriceTopic {
    pub symbol: String,
}

impl IndexPriceTopic {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FairPriceTopic {
    pub symbol: String,
}

impl FairPriceTopic {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
        }
    }
}
