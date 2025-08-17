use std::sync::Arc;

use axum::{Extension, Json, Router, response::IntoResponse, routing::get};
use chrono::Utc;
use serde_json::json;

use crate::AppState;

pub fn health_router() -> Router {
    return Router::new().route("/ping", get(ping));
}

pub async fn ping(Extension(_app_state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let now = Utc::now();
    return Json(json!({
        "response": "pong!",
        "at": now
    }));
}
