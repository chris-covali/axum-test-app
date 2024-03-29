use sqlx::MySqlPool;
use crate::models::kline_repo::KlineRepo;

pub struct Repos {
    pub kline_repo: KlineRepo,
}

impl Repos {
    pub fn new(db: MySqlPool) -> Self {
        Self {
            kline_repo: KlineRepo { db }
        }
    }
}