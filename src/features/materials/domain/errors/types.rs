use axum::http::StatusCode;

use serde_json::{json, Value};
use std::{error::Error, fmt};

// Тип для результатов API
pub type MaterialResult<T> = Result<T, MaterialError>;

#[derive(Debug, Clone)]
pub enum MaterialError {
    // Ошибки валидации материалов
    ValidationError {

        message: String,
    },

    // Ошибки поиска и существования
    NotFoundError {
        message: String,
    },

    // Ошибки дублирования
    DuplicateError {
        message: String,
    },

    // Ошибки базы данных
    DatabaseError {
        message: String,
    },

    // Ошибки репозитория
    RepositoryError {
        operation: String,
        message: String,
    },
    MappingError {
        from: String,
        to: String,
        message: String,
    },

    // Системные ошибки
    ConfigurationError {
        component: String,
        message: String,
    },
    InternalError {
        message: String,
    },
}

impl fmt::Display for MaterialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaterialError::ValidationError {  message } => {
                write!(f, "Ошибка валидации '{}'",  message)
            }

            MaterialError::NotFoundError { message } => {
                write!(f, "Ошибка поиска: {}", message)
            }

            MaterialError::DuplicateError { message } => {
                write!(f, "Ошибка дублирования: {}", message)
            }
            MaterialError::DatabaseError { message } => {
                write!(f, "Ошибка базы данных: {}", message)
            }

            MaterialError::RepositoryError { operation, message } => {
                write!(
                    f,
                    "Ошибка репозитория при операции '{}': {}",
                    operation, message
                )
            }
            MaterialError::MappingError { from, to, message } => {
                write!(f, "Ошибка маппинга {} -> {}: {}", from, to, message)
            }
            MaterialError::ConfigurationError { component, message } => {
                write!(
                    f,
                    "Ошибка конфигурации компонента '{}': {}",
                    component, message
                )
            }
            MaterialError::InternalError { message } => {
                write!(f, "Внутренняя ошибка: {}", message)
            }
        }
    }
}

impl Error for MaterialError {}

impl MaterialError {
    pub fn error_code(&self) -> &'static str {
        match self {
            MaterialError::ValidationError { .. } => "MATERIAL_VALIDATION_ERROR",

            MaterialError::NotFoundError { .. } => "MATERIAL_NOT_FOUND",

            MaterialError::DuplicateError { .. } => "MATERIAL_DUPLICATE",

            MaterialError::DatabaseError { .. } => "MATERIAL_DATABASE_ERROR",

            MaterialError::RepositoryError { .. } | MaterialError::MappingError { .. } => {
                "MATERIAL_REPOSITORY_ERROR"
            }

            MaterialError::ConfigurationError { .. } | MaterialError::InternalError { .. } => {
                "MATERIAL_INTERNAL_ERROR"
            }
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            MaterialError::ValidationError { .. } => StatusCode::BAD_REQUEST,

            MaterialError::NotFoundError { .. } => StatusCode::NOT_FOUND,

            MaterialError::DuplicateError { .. } => StatusCode::CONFLICT,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn details(&self) -> Value {
        match self {
            // MaterialError::ValidationError {  message } => json!({
     
            //     "message": message
            // }),

            // MaterialError::RepositoryError { operation, message } => json!({
            //     "operation": operation,
            //     "message": message
            // }),
            // MaterialError::MappingError { from, to, message } => json!({
            //     "from": from,
            //     "to": to,
            //     "message": message
            // }),
            // MaterialError::ConfigurationError { component, message } => json!({
            //     "component": component,
            //     "message": message
            // }),
            _ => json!({}),
        }
    }
}
