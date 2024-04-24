pub mod config;

use sqlx::mysql::MySqlPool;

use crate::models::repos::Repos;
use crate::service::Services;
use crate::state::config::Config;

pub struct AppState {
    pub config: Config,
    pub db: MySqlPool,
    pub repos: Repos,
    pub services: Services,
}

pub async fn create_app_state() -> AppState {
    let config = Config::from_env();
    let pool = MySqlPool::connect(config.database_url.as_str())
        .await
        .expect("Failed to connect to database");
    let repos = Repos::new(pool.clone());
    let services = Services::new();

    AppState {
        db: pool,
        repos,
        services,
        config,
    }
}
