use std::sync::Arc;

use axum::{Extension, Json, Router, extract::Query, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    AppState, error::http_error::HttpError, repositories::user_repository::UserRepository,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct PageParams {
    pub page: u32,
    pub limit: usize,
}

pub fn user_router() -> Router {
    return Router::new().route("/", get(find_all));
}

pub async fn find_all(
    Extension(app_state): Extension<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<impl IntoResponse, HttpError> {
    let page = app_state
        .db_client
        .find_all(params.page, params.limit)
        .await
        .map_err(|e| HttpError::internal_server_error(e.to_string()))?;

    let response = json!({
        "page": params.page,
        "size": params.limit,
        "content": page
    });

    return Ok(Json(response));
}
