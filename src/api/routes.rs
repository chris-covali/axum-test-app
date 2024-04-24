use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

use crate::state::AppState;

pub async fn hello_world() -> &'static str {
    "Hello, World!"
}

pub async fn not_found() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json::from(serde_json::json!({
            "error": "Not Found"
        })),
    )
}

pub async fn get_latest_klines(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let limit = params
        .get("limit")
        .unwrap_or(&"10".to_string())
        .parse::<i64>()
        .unwrap();
    let offset = params
        .get("offset")
        .unwrap_or(&"0".to_string())
        .parse::<i64>()
        .unwrap();

    let klines = state.repos.kline_repo.find_all_klines(limit, offset).await;

    if klines.is_empty() {
        return not_found().await;
    }

    (StatusCode::OK, Json::from(serde_json::json!(klines)))
}

pub async fn get_klines_for_pair(
    State(state): State<Arc<AppState>>,
    Path(path_params): Path<HashMap<String, String>>,
    Query(query_params): Query<HashMap<String, String>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let symbol = path_params.get("symbol").unwrap();
    let interval = path_params.get("interval").unwrap();

    let limit = query_params
        .get("limit")
        .unwrap_or(&"10".to_string())
        .parse::<i64>()
        .unwrap();
    let offset = query_params
        .get("offset")
        .unwrap_or(&"0".to_string())
        .parse::<i64>()
        .unwrap();

    let klines = state
        .repos
        .kline_repo
        .find_klines(symbol.to_string(), interval.to_string(), limit, offset)
        .await;

    if klines.is_empty() {
        return not_found().await;
    }

    (StatusCode::OK, Json::from(serde_json::json!(klines)))
}
