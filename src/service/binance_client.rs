use chrono::DateTime;
use crate::models::kline::Kline;
use crate::types::DateTimeUTC;

pub struct BinanceClient {
    base_url: String,
}

impl BinanceClient {
    pub fn prod() -> Self {
        Self {
            base_url: "https://api.binance.com".to_string(),
        }
    }

    pub async fn get_klines(&self, symbol: &str, interval: &str, limit: u16, start_time: Option<DateTimeUTC>) -> Result<Vec<Kline>, reqwest::Error> {
        let mut url = format!("{}/api/v3/klines?symbol={}&interval={}&limit={}", self.base_url, symbol, interval, limit);

        if let Some(start_time) = start_time {
            let start_time = start_time.timestamp_millis();
            url = format!("{}&startTime={}", url, start_time);
        }

        let response = reqwest::get(&url).await?.json::<Vec<serde_json::Value>>().await?;
        let items = response
            .into_iter()
            .map(|val| Self::map_val_to_kline(val, symbol.to_string(), interval.to_string()))
            .collect();

        return Ok(items);
    }

    fn map_val_to_kline(val: serde_json::Value, symbol: String, interval: String) -> Kline {
        let kline = Kline {
            id: 0,
            symbol,
            interval,
            open_time: DateTime::from_timestamp_millis(val[0].as_i64().unwrap()).unwrap(),
            open: val[1].as_str().unwrap().parse::<f64>().unwrap(),
            high: val[2].as_str().unwrap().parse::<f64>().unwrap(),
            low: val[3].as_str().unwrap().parse::<f64>().unwrap(),
            close: val[4].as_str().unwrap().parse::<f64>().unwrap(),
            volume: val[5].as_str().unwrap().parse::<f64>().unwrap(),
            close_time: DateTime::from_timestamp_millis(val[6].as_i64().unwrap()).unwrap(),
            quote_asset_volume: val[7].as_str().unwrap().parse::<f64>().unwrap(),
            number_of_trades: val[8].as_u64().unwrap(),
            taker_buy_base_asset_volume: val[9].as_str().unwrap().parse::<f64>().unwrap(),
            taker_buy_quote_asset_volume: val[10].as_str().unwrap().parse::<f64>().unwrap(),
        };

        return kline;
    }
}
