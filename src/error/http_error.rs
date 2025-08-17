use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        return Self {
            message: message.into(),
            status,
        };
    }

    pub fn internal_server_error(message: impl Into<String>) -> Self {
        return Self {
            message: message.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        };
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "error": self.message,
        });

        (self.status, axum::Json(body)).into_response()
    }
}
