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

pub fn solve(s: String) -> Vec<String> {
    let mut words: Vec<String> = s.split(" ").into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let len = words.len();

    for i in 0..len {
        let cur_wrd = words.get(i).unwrap();
        let nxt_wrd = words.get(len % (i + i)).unwrap();
        let prv_idx = if i == 0 { len - 1 } else { i - 1 };
        let prv_wrd = words.get((len % (i + 1)) - 1).unwrap();
    }

    return result;
}
