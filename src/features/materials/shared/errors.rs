use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;
use std::{error::Error, fmt};

/// Общий тип результата для всех материальных фич
pub type MaterialResult<T> = Result<T, MaterialError>;

/// Перечисление всех возможных ошибок
#[derive(Debug, Clone)]
pub enum MaterialError {
    ValidationError { message: String },
    NotFoundError { message: String },
    DuplicateError { message: String },
    DatabaseError { message: String },
    InternalError { message: String },
}

impl fmt::Display for MaterialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaterialError::ValidationError { message } => write!(f, "Ошибка валидации: {}", message),
            MaterialError::NotFoundError { message } => write!(f, "Не найдено: {}", message),
            MaterialError::DuplicateError { message } => write!(f, "Дублирование: {}", message),
            MaterialError::DatabaseError { message } => write!(f, "Ошибка БД: {}", message),
            MaterialError::InternalError { message } => write!(f, "Внутренняя ошибка: {}", message),
        }
    }
}

impl Error for MaterialError {}

/// Структура для JSON ответа с ошибкой
#[derive(Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: String,
    timestamp: String,
}

impl MaterialError {
    pub fn error_code(&self) -> &'static str {
        match self {
            MaterialError::ValidationError { .. } => "VALIDATION_ERROR",
            MaterialError::NotFoundError { .. } => "NOT_FOUND",
            MaterialError::DuplicateError { .. } => "DUPLICATE_ERROR",
            MaterialError::DatabaseError { .. } => "DATABASE_ERROR",
            MaterialError::InternalError { .. } => "INTERNAL_ERROR",
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            MaterialError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            MaterialError::NotFoundError { .. } => StatusCode::NOT_FOUND,
            MaterialError::DuplicateError { .. } => StatusCode::CONFLICT,
            MaterialError::DatabaseError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            MaterialError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for MaterialError {
    fn into_response(self) -> Response {
        let error_response = ErrorResponse {
            code: self.error_code(),
            message: self.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        (self.status_code(), Json(error_response)).into_response()
    }
}