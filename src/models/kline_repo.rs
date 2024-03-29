use crate::models::Kline;

pub struct KlineRepo {
    pub db: sqlx::MySqlPool,
}

impl KlineRepo {
    pub async fn find_all_klines(&self, limit: i64, offset: i64) -> Vec<Kline> {
        let rows = sqlx::query("SELECT * FROM kline ORDER BY open_time DESC LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.db)
            .await
            .unwrap();

        return rows.iter().map(|row| { row.into() }).collect();
    }

    pub async fn find_latest_kline_by_symbol(&self, symbol: String, interval: String) -> Option<Kline> {
        let rows = sqlx::query("SELECT * FROM kline WHERE symbol = ? AND kline.`interval` = ? ORDER BY open_time DESC LIMIT 1")
            .bind(symbol)
            .bind(interval)
            .fetch_all(&self.db)
            .await
            .unwrap();

        return rows.first().map(|row| { row.into() });
    }

    pub async fn find_klines(&self, symbol: String, interval: String, limit: i64, offset: i64) -> Vec<Kline> {
        let rows = sqlx::query("SELECT * FROM kline WHERE symbol = ? AND kline.`interval` = ? ORDER BY open_time DESC LIMIT ? OFFSET ?")
            .bind(symbol)
            .bind(interval)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.db)
            .await
            .unwrap();

        return rows.iter().map(|row| { row.into() }).collect();
    }

    pub async fn create_or_update_klines(&self, klines: &Vec<Kline>) {
        for kline in klines {
            sqlx::query(r"
                INSERT INTO kline(symbol, `interval`, open_time, open, high, low, close, volume, close_time, quote_asset_volume, number_of_trades, taker_buy_base_asset_volume, taker_buy_quote_asset_volume)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                on DUPLICATE KEY UPDATE open = VALUES(open), high = VALUES(high), low = VALUES(low), close = VALUES(close), volume = VALUES(volume), close_time = VALUES(close_time), quote_asset_volume = VALUES(quote_asset_volume), number_of_trades = VALUES(number_of_trades), taker_buy_base_asset_volume = VALUES(taker_buy_base_asset_volume), taker_buy_quote_asset_volume = VALUES(taker_buy_quote_asset_volume)
                ")
                .bind(kline.symbol.clone())
                .bind(kline.interval.clone())
                .bind(kline.open_time)
                .bind(kline.open)
                .bind(kline.high)
                .bind(kline.low)
                .bind(kline.close)
                .bind(kline.volume)
                .bind(kline.close_time)
                .bind(kline.quote_asset_volume)
                .bind(kline.number_of_trades)
                .bind(kline.taker_buy_base_asset_volume)
                .bind(kline.taker_buy_quote_asset_volume)
                .execute(&self.db)
                .await
                .unwrap();
        }
    }
}

