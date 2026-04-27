use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

/// Application-level errors stay readable in the core logic.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found")]
    NotFound,

    #[error("invalid input: {0}")]
    BadRequest(String),

    #[error("internal error")]
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

/// HTTP mapping happens at the boundary.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "BAD_REQUEST"),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL"),
        };

        let body = ErrorBody {
            code,
            message: self.to_string(),
        };

        (status, Json(body)).into_response()
    }
}
