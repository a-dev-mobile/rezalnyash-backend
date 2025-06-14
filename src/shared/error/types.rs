// src/error/types.rs - Определение типов ошибок
use axum::http::StatusCode;
use serde_json::json;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AppError {
    // Критичные системные ошибки
    ConfigurationError { message: String },
    EnvironmentError { variable: String },
    InternalError { message: String },
    DatabaseConnectionError { message: String },
    MigrationError { message: String },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ConfigurationError { message } => {
                write!(f, "Ошибка конфигурации: {}", message)
            }
            AppError::EnvironmentError { variable } => {
                write!(f, "Переменная окружения '{}' не установлена", variable)
            }
            AppError::InternalError { message } => {
                write!(f, "Внутренняя ошибка: {}", message)
            }
            AppError::DatabaseConnectionError { message } => {
                write!(f, "Ошибка подключения к базе данных: {}", message)
            }
            AppError::MigrationError { message } => {
                write!(f, "Ошибка миграции базы данных: {}", message)
            }
        }
    }
}

impl std::error::Error for AppError {}

impl AppError {
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::ConfigurationError { .. } | 
            AppError::EnvironmentError { .. } | 
            AppError::InternalError { .. } |
            AppError::DatabaseConnectionError { .. } |
            AppError::MigrationError { .. } => "SYSTEM_ERROR",
        }
    }

    pub fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    pub fn details(&self) -> serde_json::Value {
        match self {
            AppError::EnvironmentError { variable } => json!({
                "variable": variable
            }),
            AppError::DatabaseConnectionError { message } => json!({
                "message": message
            }),
            _ => json!({}),
        }
    }
}
