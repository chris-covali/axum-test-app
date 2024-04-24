use axum_test_app::state::create_app_state;
use axum_test_app::types::DateTimeUTC;

use std::str::FromStr;

const LIMIT: u16 = 1000;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let app = create_app_state().await;
    let binance_client = &app.services.binance_client;
    let mut start_time = Some(DateTimeUTC::from_str("2024-01-01T00:00:00Z").unwrap());

    loop {
        let klines = binance_client
            .get_klines("BTCUSDT", "1m", LIMIT, start_time)
            .await?;
        app.repos.kline_repo.create_or_update_klines(&klines).await;

        start_time = klines.last().map(|k| k.open_time);

        println!("Fetched data until: {:?}", start_time.unwrap().to_rfc3339());

        if klines.len() < LIMIT as usize {
            break;
        }
    }

    println!("Finished fetching data");

    Ok(())
}
