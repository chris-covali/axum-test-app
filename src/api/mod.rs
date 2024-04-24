use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use crate::state;

mod routes;

pub async fn create_web_app() -> Router {
    let state = state::create_app_state().await;

    Router::new()
        .route("/", get(routes::hello_world))
        .route("/klines", get(routes::get_latest_klines))
        .route(
            "/klines/:symbol/:interval",
            get(routes::get_klines_for_pair),
        )
        .with_state(Arc::new(state))
        .fallback(routes::not_found)
}
