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
    services::user_service, utils::page_params::PageParams,
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
    let opt_users = user_service::find_all(Extension(app_state), Query(params)).await;

    return Ok(Json(json!(opt_users.unwrap())));
}

pub async fn find_by_id(
    Extension(app_state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, HttpError> {
    let user = user_service::find_by_id(Extension(app_state), Path(id)).await;
    return match user {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(err),
    };
}
