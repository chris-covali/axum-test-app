use std::sync::Arc;

use axum::Router;
use axum::routing::get;

use crate::state;

mod routes;

pub async fn create_web_app() -> Router {
    let state = state::create_app_state().await;

    let app = Router::new()
        .route("/", get(routes::hello_world))
        .route("/klines", get(routes::get_latest_klines))
        .route("/klines/:symbol/:interval", get(routes::get_klines_for_pair))
        .with_state(Arc::new(state))
        .fallback(routes::not_found);

    return app;
}