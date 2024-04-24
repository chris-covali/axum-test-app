use crate::models::Kline;

pub struct KlineRepo {
    pub db: sqlx::MySqlPool,
}

impl KlineRepo {
    pub async fn find_all_klines(&self, limit: i64, offset: i64) -> Vec<Kline> {
        sqlx::query_file_as!(Kline, "src/models/queries/fetch_all.sql", limit, offset)
            .fetch_all(&self.db)
            .await
            .unwrap()
    }

    pub async fn find_latest_kline_by_symbol(
        &self,
        symbol: String,
        interval: String,
    ) -> Option<Kline> {
        sqlx::query_file_as!(
            Kline,
            "src/models/queries/fetch_latest_by_symbol.sql",
            symbol,
            interval
        )
        .fetch_one(&self.db)
        .await
        .ok()
    }

    pub async fn find_klines(
        &self,
        symbol: String,
        interval: String,
        limit: i64,
        offset: i64,
    ) -> Vec<Kline> {
        sqlx::query_file_as!(
            Kline,
            "src/models/queries/fetch_by_symbol.sql",
            symbol,
            interval,
            limit,
            offset,
        )
        .fetch_all(&self.db)
        .await
        .unwrap()
    }

    pub async fn create_or_update_klines(&self, klines: &Vec<Kline>) {
        for kline in klines {
            sqlx::query_file!(
                "src/models/queries/insert_or_update.sql",
                kline.symbol,
                kline.interval,
                kline.open_time,
                kline.open,
                kline.high,
                kline.low,
                kline.close,
                kline.volume,
                kline.close_time,
                kline.quote_asset_volume,
                kline.number_of_trades,
                kline.taker_buy_base_asset_volume,
                kline.taker_buy_quote_asset_volume
            )
            .execute(&self.db)
            .await
            .unwrap();
        }
    }
}
