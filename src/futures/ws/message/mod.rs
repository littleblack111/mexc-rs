use serde::Deserialize;

#[derive(Debug)]
pub enum FuturesMessage {
    Tickers(TickersMessage),
    Ticker(TickerMessage),
    Deal(DealMessage),
    Depth(DepthMessage),
    DepthStep(DepthStepMessage),
    Kline(KlineMessage),
    FundingRate(FundingRateMessage),
    IndexPrice(IndexPriceMessage),
    FairPrice(FairPriceMessage),
    Contract(Box<ContractMessage>),
    EventContract(EventContractMessage),
    PersonalOrder(PersonalOrderMessage),
    PersonalAsset(PersonalAssetMessage),
    PersonalPosition(PersonalPositionMessage),
    PersonalPlanOrder(PersonalPlanOrderMessage),
    PersonalRiskLimit(PersonalRiskLimitMessage),
    PersonalStopPlanorder(PersonalStopPlanorderMessage),
    PersonalTrackOrder(PersonalTrackOrderMessage),
    PersonalStopOrder(PersonalStopOrderMessage),
    PersonalOrderDeal(PersonalOrderDealMessage),
    PersonalLiquidateRisk(PersonalLiquidateRiskMessage),
    PersonalLeverageMode(PersonalLeverageModeMessage),
    PersonalPositionMode(PersonalPositionModeMessage),
    PersonalReversePosition(PersonalReversePositionMessage),
    PersonalBonus(PersonalBonusMessage),
    PersonalEventContractPosition(PersonalEventContractPositionMessage),
    PersonalGenericNotify(PersonalGenericNotifyMessage),
    PersonalOrderChase(PersonalOrderChaseMessage),
    PersonalPositionCloseallFail(PersonalPositionCloseallFailMessage),
    Pong(PongMessage),
    LoginResponse(LoginResponseMessage),
    ErrorMsg(ErrorMessage),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum RawFuturesMessage {
    Pong(PongMessage),
    LoginResponse(LoginResponseMessage),
    ErrorMsg(ErrorMessage),
    PushMessage(RawPushMessage),
}

#[derive(Debug, Deserialize)]
pub struct RawPushMessage {
    pub channel: String,
    pub data: serde_json::Value,
    pub symbol: Option<String>,
    pub ts: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PongMessage {
    pub channel: String,
    pub data: i64,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponseMessage {
    pub channel: String,
    pub data: String,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
pub struct ErrorMessage {
    pub channel: String,
    pub code: i32,
    pub msg: String,
    pub ts: i64,
}

impl TryFrom<RawFuturesMessage> for FuturesMessage {
    type Error = ();

    fn try_from(value: RawFuturesMessage) -> Result<Self, Self::Error> {
        match value {
            RawFuturesMessage::PushMessage(raw) => {
                match raw
                    .channel
                    .as_str()
                {
                    "push.tickers" => {
                        let data: Vec<TickerData> = serde_json::from_value(raw.data).map_err(|_| ())?;
                        Ok(
                            FuturesMessage::Tickers(
                                TickersMessage {
                                    data,
                                },
                            ),
                        )
                    }
                    "push.ticker" => {
                        let data: TickerData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let symbol = raw
                            .symbol
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::Ticker(
                                TickerMessage {
                                    data,
                                    symbol,
                                },
                            ),
                        )
                    }
                    "push.deal" => {
                        let data: Vec<DealData> = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let symbol = raw
                            .symbol
                            .unwrap_or_default();
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::Deal(
                                DealMessage {
                                    symbol,
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.depth" => {
                        let data: DepthData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let symbol = raw
                            .symbol
                            .unwrap_or_default();
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::Depth(
                                DepthMessage {
                                    data,
                                    symbol,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.depth.step" => {
                        let data: DepthStepData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let symbol = raw
                            .symbol
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::DepthStep(
                                DepthStepMessage {
                                    data,
                                    symbol,
                                },
                            ),
                        )
                    }
                    "push.kline" => {
                        let data: KlineData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let symbol = raw
                            .symbol
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::Kline(
                                KlineMessage {
                                    data,
                                    symbol,
                                },
                            ),
                        )
                    }
                    "push.funding.rate" => {
                        let data: FundingRateData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let symbol = raw
                            .symbol
                            .unwrap_or_default();
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::FundingRate(
                                FundingRateMessage {
                                    data,
                                    symbol,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.index.price" => {
                        let data: IndexPriceData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let symbol = raw
                            .symbol
                            .unwrap_or_default();
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::IndexPrice(
                                IndexPriceMessage {
                                    data,
                                    symbol,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.fair.price" => {
                        let data: FairPriceData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let symbol = raw
                            .symbol
                            .unwrap_or_default();
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::FairPrice(
                                FairPriceMessage {
                                    data,
                                    symbol,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.contract" => {
                        let data: ContractData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let symbol = raw
                            .symbol
                            .unwrap_or_default();
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::Contract(
                                Box::new(
                                    ContractMessage {
                                        data,
                                        symbol,
                                        ts,
                                    },
                                ),
                            ),
                        )
                    }
                    "push.event.contract" => {
                        let data: EventContractData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let symbol = raw
                            .symbol
                            .unwrap_or_default();
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::EventContract(
                                EventContractMessage {
                                    data,
                                    symbol,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.order" => {
                        let data: PersonalOrderData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalOrder(
                                PersonalOrderMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.asset" => {
                        let data: PersonalAssetData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalAsset(
                                PersonalAssetMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.position" => {
                        let data: PersonalPositionData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalPosition(
                                PersonalPositionMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.plan.order" => {
                        let data: PersonalPlanOrderData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalPlanOrder(
                                PersonalPlanOrderMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.risk.limit" => {
                        let data: PersonalRiskLimitData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalRiskLimit(
                                PersonalRiskLimitMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.stop.planorder" => {
                        let data: PersonalStopPlanorderData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalStopPlanorder(
                                PersonalStopPlanorderMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.track.order" => {
                        let data: PersonalTrackOrderData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalTrackOrder(
                                PersonalTrackOrderMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.stop.order" => {
                        let data: PersonalStopOrderData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalStopOrder(
                                PersonalStopOrderMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.order.deal" => {
                        let data: PersonalOrderDealData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalOrderDeal(
                                PersonalOrderDealMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.liquidate.risk" => {
                        let data: PersonalLiquidateRiskData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalLiquidateRisk(
                                PersonalLiquidateRiskMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.leverage.mode" => {
                        let data: PersonalLeverageModeData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalLeverageMode(
                                PersonalLeverageModeMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.position.mode" => {
                        let data: PersonalPositionModeData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalPositionMode(
                                PersonalPositionModeMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.reverse.position" => {
                        let data: PersonalReversePositionData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalReversePosition(
                                PersonalReversePositionMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.bonus" => {
                        let data: PersonalBonusData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalBonus(
                                PersonalBonusMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.event.contract.position" => {
                        let data: PersonalEventContractPositionData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalEventContractPosition(
                                PersonalEventContractPositionMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.generic.notify" => {
                        let data: PersonalGenericNotifyData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalGenericNotify(
                                PersonalGenericNotifyMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.order.chase" => {
                        let data: PersonalOrderChaseData = serde_json::from_value(raw.data).map_err(|_| ())?;
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalOrderChase(
                                PersonalOrderChaseMessage {
                                    data,
                                    ts,
                                },
                            ),
                        )
                    }
                    "push.personal.position.closeall.fail" => {
                        let ts = raw
                            .ts
                            .unwrap_or_default();
                        Ok(
                            FuturesMessage::PersonalPositionCloseallFail(
                                PersonalPositionCloseallFailMessage {
                                    ts,
                                },
                            ),
                        )
                    }
                    _ => Err(()),
                }
            }
            RawFuturesMessage::Pong(pong) => Ok(FuturesMessage::Pong(pong)),
            RawFuturesMessage::LoginResponse(login) => Ok(FuturesMessage::LoginResponse(login)),
            RawFuturesMessage::ErrorMsg(error) => Ok(FuturesMessage::ErrorMsg(error)),
        }
    }
}

// Define the message structs here for simplicity

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickersMessage {
    pub data: Vec<TickerData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerData {
    pub symbol: String,
    pub timestamp: Option<i64>,
    pub last_price: Option<f64>,
    pub bid1: Option<f64>,
    pub ask1: Option<f64>,
    pub hold_vol: Option<f64>,
    pub funding_rate: Option<f64>,
    pub rise_fall_rate: Option<f64>,
    pub rise_fall_value: Option<f64>,
    pub volume24: Option<f64>,
    pub amount24: Option<f64>,
    pub fair_price: Option<f64>,
    pub index_price: Option<f64>,
    pub max_bid_price: Option<f64>,
    pub min_ask_price: Option<f64>,
    pub lower24_price: Option<f64>,
    pub high24_price: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerMessage {
    pub data: TickerData,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealMessage {
    pub symbol: String,
    pub data: Vec<DealData>,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
pub struct DealData {
    pub p: f64,
    pub v: f64,
    #[serde(rename = "T")]
    pub trade_side: i32,
    #[serde(rename = "O")]
    pub open_close_flag: i32,
    #[serde(rename = "M")]
    pub self_trade: i32,
    #[serde(rename = "t")]
    pub trade_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthMessage {
    pub data: DepthData,
    pub symbol: String,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
pub struct DepthData {
    pub asks: Vec<DepthLevel>,
    pub bids: Vec<DepthLevel>,
    pub version: i64,
}

#[derive(Debug, Deserialize)]
pub struct DepthLevel(
    pub f64,
    pub f64,
    pub f64,
);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthStepMessage {
    pub data: DepthStepData,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthStepData {
    pub ask_market_level_price: f64,
    pub asks: Vec<DepthStepLevel>,
    pub bid_market_level_price: f64,
    pub bids: Vec<DepthStepLevel>,
    pub ct: i64,
    pub version: i64,
}

#[derive(Debug, Deserialize)]
pub struct DepthStepLevel(
    pub f64,
    pub f64,
    pub f64,
);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineMessage {
    pub data: KlineData,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineData {
    pub symbol: String,
    pub interval: String,
    pub a: f64,
    pub q: f64,
    pub o: f64,
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub v: f64,
    pub ro: f64,
    pub rc: f64,
    pub rh: f64,
    pub rl: f64,
    pub t: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateMessage {
    pub data: FundingRateData,
    pub symbol: String,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
pub struct FundingRateData {
    pub rate: f64,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceMessage {
    pub data: IndexPriceData,
    pub symbol: String,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
pub struct IndexPriceData {
    pub price: f64,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FairPriceMessage {
    pub data: FairPriceData,
    pub symbol: String,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
pub struct FairPriceData {
    pub price: f64,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractMessage {
    pub data: ContractData,
    pub symbol: String,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractData {
    pub symbol: String,
    pub display_name: String,
    pub display_name_en: String,
    pub position_open_type: i32,
    pub base_coin: String,
    pub quote_coin: String,
    pub base_coin_name: String,
    pub quote_coin_name: String,
    pub future_type: i32,
    pub settle_coin: String,
    pub contract_size: f64,
    pub min_leverage: i32,
    pub max_leverage: i32,
    pub price_scale: i32,
    pub vol_scale: i32,
    pub amount_scale: i32,
    pub price_unit: f64,
    pub vol_unit: f64,
    pub min_vol: f64,
    pub max_vol: f64,
    pub limit_max_vol: f64,
    pub bid_limit_price_rate: f64,
    pub ask_limit_price_rate: f64,
    pub taker_fee_rate: f64,
    pub maker_fee_rate: f64,
    pub maintenance_margin_rate: f64,
    pub initial_margin_rate: f64,
    pub risk_base_vol: f64,
    pub risk_incr_vol: f64,
    pub risk_incr_mmr: f64,
    pub risk_incr_imr: f64,
    pub risk_level_limit: f64,
    pub price_coefficient_variation: f64,
    pub state: i32,
    pub is_new: bool,
    pub is_hot: bool,
    pub is_hidden: bool,
    pub trigger_protect: f64,
    pub risk_long_short_switch: i32,
    pub risk_base_vol_long: Option<f64>,
    pub risk_incr_vol_long: Option<f64>,
    pub risk_base_vol_short: Option<f64>,
    pub risk_incr_vol_short: Option<f64>,
    pub opening_countdown_option: i32,
    pub opening_time: i64,
    pub liquidation_fee_rate: f64,
    pub tiered_deal_amount: Option<f64>,
    pub tiered_effective_day: Option<i32>,
    pub tiered_exclude_zero_fee: Option<bool>,
    pub tiered_appoint_contract: Option<bool>,
    pub tiered_exclude_contract_id: Option<bool>,
    pub fee_rate_mode: String,
    pub index_origin: Vec<String>,
    pub concept_plate: Vec<String>,
    pub concept_plate_id: Vec<String>,
    pub depth_step_list: Vec<String>,
    pub max_num_orders: Vec<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventContractMessage {
    pub data: EventContractData,
    pub symbol: String,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventContractData {
    pub contract_id: String,
    pub symbol: String,
    pub base_coin: String,
    pub quote_coin: String,
    pub base_coin_name: String,
    pub quote_coin_name: String,
    pub settle_coin: String,
    pub base_coin_icon_url: String,
    pub invest_min_amount: f64,
    pub invest_max_amount: f64,
    pub amount_scale: i32,
    pub pay_rate_scale: i32,
    pub index_price_scale: i32,
    pub available_scale: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalOrderMessage {
    pub data: PersonalOrderData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalOrderData {
    pub order_id: i64,
    pub symbol: String,
    pub position_id: i64,
    pub price: f64,
    pub vol: f64,
    pub leverage: i64,
    pub side: i32,
    pub category: i32,
    pub order_type: i32,
    pub deal_avg_price: f64,
    pub deal_vol: f64,
    pub order_margin: f64,
    pub used_margin: f64,
    pub taker_fee: f64,
    pub maker_fee: f64,
    pub profit: f64,
    pub fee_currency: String,
    pub open_type: i32,
    pub state: i32,
    pub error_code: i32,
    pub external_oid: String,
    pub create_time: i64,
    pub update_time: i64,
    pub remain_vol: f64,
    pub position_mode: i32,
    pub reduce_only: bool,
    pub bbo_type_num: i32,
    pub maker_fee_rate: f64,
    pub taker_fee_rate: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalAssetMessage {
    pub data: PersonalAssetData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalAssetData {
    pub currency: String,
    pub position_margin: f64,
    pub frozen_balance: f64,
    pub available_balance: f64,
    pub bonus: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalPositionMessage {
    pub data: PersonalPositionData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalPositionData {
    pub position_id: i64,
    pub symbol: String,
    pub hold_vol: f64,
    pub position_type: i32,
    pub open_type: i32,
    pub state: i32,
    pub frozen_vol: f64,
    pub close_vol: f64,
    pub hold_avg_price: f64,
    pub hold_avg_price_fully_scale: String,
    pub close_avg_price: f64,
    pub open_avg_price: f64,
    pub open_avg_price_fully_scale: String,
    pub liquidate_price: f64,
    pub oim: f64,
    pub adl_level: i32,
    pub im: f64,
    pub hold_fee: f64,
    pub realised: f64,
    pub leverage: i32,
    pub auto_add_im: bool,
    pub pnl: f64,
    pub margin_ratio: f64,
    pub new_open_avg_price: f64,
    pub new_close_avg_price: f64,
    pub close_profit_loss: f64,
    pub fee: f64,
    pub deduct_fee_list: Vec<DeductFee>,
    pub maker_fee_rate: f64,
    pub taker_fee_rate: f64,
    pub create_time: i64,
    pub update_time: i64,
    pub version: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeductFee {
    pub currency: String,
    pub deduct_fee: f64,
    pub convert_settle_fee: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalPlanOrderMessage {
    pub data: PersonalPlanOrderData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalPlanOrderData {
    pub id: i64,
    pub symbol: String,
    pub leverage: i32,
    pub side: i32,
    pub trigger_price: f64,
    pub price: f64,
    pub vol: f64,
    pub open_type: i32,
    pub trigger_type: i32,
    pub state: i32,
    pub execute_cycle: i32,
    pub trend: i32,
    pub error_code: i32,
    pub order_id: i64,
    pub order_type: i32,
    pub market_order_level: i32,
    pub position_mode: i32,
    pub loss_trend: i32,
    pub profit_trend: i32,
    pub stop_loss_price: f64,
    pub take_profit_price: f64,
    pub reduce_only: bool,
    pub create_time: i64,
    pub update_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalRiskLimitMessage {
    pub data: PersonalRiskLimitData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalRiskLimitData {
    pub symbol: String,
    pub position_type: i32,
    pub risk_source: i32,
    pub level: i32,
    pub max_vol: f64,
    pub max_leverage: i32,
    pub mmr: f64,
    pub imr: f64,
    pub leverage: i32,
    pub open_type: i32,
    pub limit_by_sys: bool,
    pub max_vol_view: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalStopPlanorderMessage {
    pub data: PersonalStopPlanorderData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalStopPlanorderData {
    pub id: i64,
    pub order_id: i64,
    pub symbol: String,
    pub position_id: i64,
    pub loss_trend: i32,
    pub profit_trend: i32,
    pub stop_loss_price: f64,
    pub take_profit_price: f64,
    pub state: i32,
    pub trigger_side: i32,
    pub position_type: i32,
    pub vol: f64,
    pub take_profit_vol: f64,
    pub stop_loss_vol: f64,
    pub reality_vol: f64,
    pub place_order_id: i64,
    pub version: i32,
    pub is_finished: i32,
    pub profit_loss_vol_type: String,
    pub vol_type: i32,
    pub take_profit_reverse: i32,
    pub stop_loss_reverse: i32,
    pub close_try_times: i32,
    pub reverse_try_times: i32,
    pub reverse_error_code: i32,
    pub stop_loss_type: i32,
    pub stop_loss_order_price: f64,
    pub create_time: i64,
    pub update_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalTrackOrderMessage {
    pub data: PersonalTrackOrderData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalTrackOrderData {
    pub id: i64,
    pub symbol: String,
    pub leverage: i32,
    pub side: i32,
    pub vol: f64,
    pub open_type: i32,
    pub trend: i32,
    pub active_price: f64,
    pub mark_price: f64,
    pub back_type: i32,
    pub back_value: f64,
    pub trigger_price: f64,
    pub trigger_type: i32,
    pub order_id: i64,
    pub error_code: i32,
    pub state: i32,
    pub position_mode: i32,
    pub reduce_only: bool,
    pub create_time: i64,
    pub update_time: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalStopOrderMessage {
    pub data: PersonalStopOrderData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalStopOrderData {
    pub symbol: String,
    pub order_id: i64,
    pub loss_trend: i32,
    pub profit_trend: i32,
    pub stop_loss_price: f64,
    pub take_profit_price: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalOrderDealMessage {
    pub data: PersonalOrderDealData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalOrderDealData {
    pub id: i64,
    pub symbol: String,
    pub side: i32,
    pub vol: f64,
    pub price: f64,
    pub fee_currency: String,
    pub fee: f64,
    pub timestamp: i64,
    pub profit: f64,
    pub is_taker: bool,
    pub category: i32,
    pub order_id: i64,
    pub is_self: bool,
    pub external_oid: String,
    pub position_mode: i32,
    pub reduce_only: bool,
    pub opponent_uid: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalLiquidateRiskMessage {
    pub data: PersonalLiquidateRiskData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalLiquidateRiskData {
    pub symbol: String,
    pub position_id: i64,
    pub liquidate_price: f64,
    pub margin_ratio: f64,
    pub adl_level: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalLeverageModeMessage {
    pub data: PersonalLeverageModeData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
pub struct PersonalLeverageModeData {
    pub lm: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalPositionModeMessage {
    pub data: PersonalPositionModeData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalPositionModeData {
    pub position_mode: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalReversePositionMessage {
    pub data: PersonalReversePositionData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalReversePositionData {
    pub contract_id: i32,
    pub position_id: i64,
    pub state: i32,
    pub error_code: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalBonusMessage {
    pub data: PersonalBonusData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
pub struct PersonalBonusData {
    pub c: String,
    pub b: f64,
    pub be: i64,
    pub g: bool,
    pub ret: i64,
    pub rea: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalEventContractPositionMessage {
    pub data: PersonalEventContractPositionData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalEventContractPositionData {
    pub position_id: i64,
    pub symbol: String,
    pub side: String,
    pub pay_rate: f64,
    pub amount: f64,
    pub open_price: f64,
    pub close_price: f64,
    pub reward_amount: f64,
    pub reward_amount_usdt: f64,
    pub state: String,
    pub close_result: String,
    pub create_time: i64,
    pub close_time: i64,
    pub pnl_amount: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalGenericNotifyMessage {
    pub data: PersonalGenericNotifyData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalGenericNotifyData {
    pub r#type: i32,
    pub param: NotifyParam,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyParam {
    pub notify_type: i32,
    pub open_type: i32,
    pub dn: String,
    pub dne: String,
    pub multi_assets: bool,
    pub margin_rate: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalOrderChaseMessage {
    pub data: PersonalOrderChaseData,
    pub ts: i64,
}

#[derive(Debug, Deserialize)]
pub struct PersonalOrderChaseData {
    pub ec: i32,
    pub s: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalPositionCloseallFailMessage {
    pub ts: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ticker_message() {
        let json = r#"{
            "channel": "push.ticker",
            "data": {
                "symbol": "BTC_USDT",
                "timestamp": 1587442022003,
                "lastPrice": 6865.5,
                "bid1": 6865,
                "ask1": 6866.5,
                "holdVol": 2284742,
                "fundingRate": 0.0008,
                "riseFallRate": -0.0424,
                "riseFallValue": -304.5,
                "volume24": 164586129,
                "amount24": 123456.78,
                "fairPrice": 6867.4,
                "indexPrice": 6861.6,
                "maxBidPrice": 7073.42,
                "minAskPrice": 6661.37,
                "lower24Price": 6756,
                "high24Price": 7223.5
            },
            "symbol": "BTC_USDT"
        }"#;
        let raw: RawFuturesMessage = serde_json::from_str(json).unwrap();
        let message: FuturesMessage = raw
            .try_into()
            .unwrap();
        match message {
            FuturesMessage::Ticker(ticker) => {
                assert_eq!(
                    ticker.symbol,
                    "BTC_USDT"
                );
                assert_eq!(
                    ticker
                        .data
                        .last_price,
                    Some(6865.5)
                );
            }
            _ => panic!("Expected Ticker message"),
        }
    }

    #[test]
    fn test_parse_deal_message() {
        let json = r#"{
            "symbol": "BTC_USDT",
            "data": [
                {
                    "p": 115309.8,
                    "v": 55,
                    "T": 2,
                    "O": 3,
                    "M": 1,
                    "t": 1755487578276
                }
            ],
            "channel": "push.deal",
            "ts": 1755487578276
        }"#;
        let raw: RawFuturesMessage = serde_json::from_str(json).unwrap();
        let message: FuturesMessage = raw
            .try_into()
            .unwrap();
        match message {
            FuturesMessage::Deal(deal) => {
                assert_eq!(
                    deal.symbol,
                    "BTC_USDT"
                );
                assert_eq!(
                    deal.data
                        .len(),
                    1
                );
                assert_eq!(
                    deal.data[0].p,
                    115309.8
                );
                assert_eq!(
                    deal.data[0].trade_side,
                    2
                );
                assert_eq!(
                    deal.data[0].open_close_flag,
                    3
                );
                assert_eq!(
                    deal.data[0].self_trade,
                    1
                );
                assert_eq!(
                    deal.data[0].trade_time,
                    1755487578276
                );
            }
            _ => panic!("Expected Deal message"),
        }
    }

    #[test]
    fn test_parse_pong_message() {
        let json = r#"{
            "channel": "pong",
            "data": 1587453241453
        }"#;
        let raw: RawFuturesMessage = serde_json::from_str(json).unwrap();
        let message: FuturesMessage = raw
            .try_into()
            .unwrap();
        match message {
            FuturesMessage::Pong(pong) => {
                assert_eq!(
                    pong.data,
                    1587453241453
                );
            }
            _ => panic!("Expected Pong message"),
        }
    }

    #[test]
    fn test_parse_login_response() {
        let json = r#"{
            "channel": "rs.login",
            "data": "success",
            "ts": 1587442022003
        }"#;
        let raw: RawFuturesMessage = serde_json::from_str(json).unwrap();
        let message: FuturesMessage = raw
            .try_into()
            .unwrap();
        match message {
            FuturesMessage::LoginResponse(login) => {
                assert_eq!(
                    login.data,
                    "success"
                );
            }
            _ => panic!("Expected LoginResponse message"),
        }
    }
}
