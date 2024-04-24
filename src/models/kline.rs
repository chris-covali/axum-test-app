use serde::{Deserialize, Serialize};

use crate::types::DateTimeUTC;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Kline {
    pub id: i64,
    pub symbol: String,
    pub interval: String,
    pub open_time: DateTimeUTC,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f32,
    pub close_time: DateTimeUTC,
    pub quote_asset_volume: f32,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: f32,
    pub taker_buy_quote_asset_volume: f32,
    pub indicators: Option<serde_json::Value>,
}
