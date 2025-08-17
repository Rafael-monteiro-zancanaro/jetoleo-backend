use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    AppState, error::http_error::HttpError, repositories::user_repository::UserRepository,
    utils::page_params::PageParams,
};

pub fn user_router() -> Router {
    return Router::new()
        .route("/", get(find_all))
        .route("/{id}", get(find_by_id));
}

pub async fn find_all(
    Extension(app_state): Extension<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<impl IntoResponse, HttpError> {
    let (page, size) = (
        params.page.unwrap_or(1_u32),
        params.size.unwrap_or(25_usize),
    );
    let users = app_state
        .db_client
        .find_all(page, size)
        .await
        .map_err(|e| HttpError::internal_server_error(e.to_string()))?;

    let response = json!({
        "page": page,
        "size": size,
        "content": users
    });

    return Ok(Json(response));
}

pub async fn find_by_id(
    Extension(app_state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, HttpError> {
    let opt_user = app_state
        .db_client
        .find_by_id(id)
        .await
        .map_err(|e| HttpError::not_found(e.to_string()));

    if opt_user.is_err() {
        return Err(opt_user.unwrap_err());
    }

    return Ok(Json(opt_user.unwrap()));
}
