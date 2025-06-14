use axum::http::StatusCode;
use serde_json::{json, Value};
use std::{error::Error, fmt};

pub type HealthResult<T> = Result<T, HealthError>;

#[derive(Debug, Clone)]
pub enum HealthError {
    // Ошибки проверки здоровья
    DatabaseHealthCheckFailed {
        message: String,
    },
    ServiceUnavailable {
        service: String,
        message: String,
    },
    
    // Системные ошибки для health
    InternalError {
        message: String,
    },
}

impl fmt::Display for HealthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthError::DatabaseHealthCheckFailed { message } => {
                write!(f, "Проверка здоровья базы данных не удалась: {}", message)
            }
            HealthError::ServiceUnavailable { service, message } => {
                write!(f, "Сервис '{}' недоступен: {}", service, message)
            }
            HealthError::InternalError { message } => {
                write!(f, "Внутренняя ошибка health: {}", message)
            }
        }
    }
}

impl Error for HealthError {}

impl HealthError {
    pub fn error_code(&self) -> &'static str {
        match self {
            HealthError::DatabaseHealthCheckFailed { .. } => "HEALTH_DATABASE_ERROR",
            HealthError::ServiceUnavailable { .. } => "HEALTH_SERVICE_UNAVAILABLE",
            HealthError::InternalError { .. } => "HEALTH_INTERNAL_ERROR",
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            HealthError::DatabaseHealthCheckFailed { .. } | 
            HealthError::ServiceUnavailable { .. } => StatusCode::SERVICE_UNAVAILABLE,
            HealthError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn details(&self) -> Value {
        match self {
            HealthError::ServiceUnavailable { service, message } => json!({
                "service": service,
                "message": message
            }),
            _ => json!({}),
        }
    }
}