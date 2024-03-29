use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::Row;
use crate::types::DateTimeUTC;

#[derive(Debug, Serialize, Deserialize)]
pub struct Kline {
    pub id: u64,
    pub symbol: String,
    pub interval: String,
    pub open_time: DateTimeUTC,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: DateTimeUTC,
    pub quote_asset_volume: f64,
    pub number_of_trades: u64,
    pub taker_buy_base_asset_volume: f64,
    pub taker_buy_quote_asset_volume: f64,
}

impl From<&MySqlRow> for Kline {
    fn from(row: &MySqlRow) -> Self {
        Kline {
            id: row.get::<i64, _>("id") as u64,
            symbol: row.get("symbol"),
            interval: row.get("interval"),
            open_time: row.get::<DateTimeUTC, _>("open_time"),
            close_time: row.get::<DateTimeUTC, _>("close_time"),
            open: row.get::<f32, _>("open") as f64,
            high: row.get::<f32, _>("high") as f64,
            low: row.get::<f32, _>("low") as f64,
            close: row.get::<f32, _>("close") as f64,
            volume: row.get::<f32, _>("volume") as f64,
            number_of_trades: row.get::<i32, _>("number_of_trades") as u64,
            quote_asset_volume: row.get::<f32, _>("quote_asset_volume") as f64,
            taker_buy_base_asset_volume: row.get::<f32, _>("taker_buy_base_asset_volume") as f64,
            taker_buy_quote_asset_volume: row.get::<f32, _>("taker_buy_quote_asset_volume") as f64,
        }

    }
}