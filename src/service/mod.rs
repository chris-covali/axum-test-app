use crate::service::binance_client::BinanceClient;

pub mod binance_client;

pub struct Services {
    pub binance_client: BinanceClient,
}

impl Services {
    pub fn new() -> Services {
        Services {
            binance_client: BinanceClient::prod(),
        }
    }
}
