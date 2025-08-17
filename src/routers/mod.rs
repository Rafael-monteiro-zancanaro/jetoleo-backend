mod health_router;

use std::sync::Arc;

use axum::{Extension, Router};
use tower_http::trace::TraceLayer;

use crate::{AppState, routers::health_router::health_router};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_route = Router::new()
        .nest("/health", health_router())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));

    return Router::new().nest("/api", api_route);
}
