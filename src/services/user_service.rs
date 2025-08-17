use std::sync::Arc;

use axum::{
    Extension,
    extract::{Path, Query},
};
use uuid::Uuid;

use crate::{
    AppState, error::http_error::HttpError, models::user::User,
    repositories::user_repository::UserRepository, utils::page_params::PageParams,
};

pub async fn find_all(
    Extension(app_state): Extension<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Vec<User>, HttpError> {
    let (page, size) = (
        params.page.unwrap_or(1_u32),
        params.size.unwrap_or(25_usize),
    );
    let users = app_state
        .db_client
        .find_all(page, size)
        .await
        .map_err(|e| HttpError::internal_server_error(e.to_string()))?;

    return Ok(users);
}

pub async fn find_by_id(
    Extension(app_state): Extension<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<User, HttpError> {
    let opt_user = app_state
        .db_client
        .find_by_id(id)
        .await
        .map_err(|e| HttpError::not_found(e.to_string()));

    if opt_user.is_err() {
        return Err(opt_user.unwrap_err());
    }

    if let Some(user) = opt_user.unwrap() {
        return Ok(user);
    }

    return Err(HttpError::not_found(format!(
        "Could not find user with id {}",
        id
    )));
}
