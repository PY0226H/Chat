use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("jwt. error: {0}")]
    JwtError(#[from] jwt_simple::Error),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

impl ErrorOutput {
    pub fn new(error: impl Into<String>) -> Self {
        ErrorOutput {
            error: error.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response<axum::body::Body> {
        let status = match self {
            AppError::JwtError(_) => axum::http::StatusCode::FORBIDDEN,
            AppError::IoError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
