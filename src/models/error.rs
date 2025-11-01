use std::fmt::Display;
use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;
use tracing::error;

#[derive(Debug, Serialize, Clone)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let ErrorResponse { status, message } = self;
        error!("{}", &message);
        let body = axum::Json(ErrorResponse {
            status,
            message: message.clone(),
        });
        (StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR), body).into_response()
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
