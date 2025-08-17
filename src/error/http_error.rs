use axum::http::StatusCode;

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

    pub fn internal_server_error(message: impl Into<String>, status: StatusCode) -> Self {
        return Self {
            message: message.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        };
    }
}
