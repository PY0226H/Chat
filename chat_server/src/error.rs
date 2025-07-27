use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("user already exists: {0}")]
    UserAlreadyExists(String),

    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("create chat error: {0}")]
    CreateChatError(String),

    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("jwt. error: {0}")]
    JwtError(#[from] jwt_simple::Error),

    #[error("http header parse error: {0}")]
    HttpHeaderError(#[from] axum::http::header::InvalidHeaderValue),

    #[error("not found: {0}")]
    NotFound(String),

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
            AppError::SqlxError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordHashError(_) => axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            AppError::JwtError(_) => axum::http::StatusCode::FORBIDDEN,
            AppError::HttpHeaderError(_) => axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            AppError::UserAlreadyExists(_) => axum::http::StatusCode::CONFLICT,
            AppError::CreateChatError(_) => axum::http::StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            AppError::IoError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
