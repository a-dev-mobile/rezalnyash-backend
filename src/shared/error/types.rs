// src/error/types.rs - Определение типов ошибок
use axum::http::StatusCode;
use serde_json::json;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AppError {
    // Ошибки валидации (для API)
    ValidationError { field: String, message: String },
    InvalidDimensions { width: f64, height: f64 },
    InvalidQuantity { quantity: i32 },
    EmptySheets,
    EmptyParts,

    // Ошибки расчета (бизнес-логика)
    CalculationError { message: String },
    PartsTooBig { part_ids: Vec<String> },
    NoSolution,
    OptimizationFailed,

    // Ошибки данных (для API)
    NotFound { resource: String, id: String },
    DatabaseError { message: String },
    MigrationError { message: String },

    // Ошибки экспорта (для API)
    ExportError { format: String, message: String },
    PdfGenerationFailed,
    DxfGenerationFailed,

    // Критичные системные ошибки
    ConfigurationError { message: String },
    EnvironmentError { variable: String },
    InternalError { message: String },
    
    // Тестовая ошибка
    TestError { message: String },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ValidationError { field, message } => {
                write!(f, "Ошибка валидации поля '{}': {}", field, message)
            }
            AppError::InvalidDimensions { width, height } => {
                write!(f, "Некорректные размеры: {}x{} мм", width, height)
            }
            AppError::InvalidQuantity { quantity } => {
                write!(f, "Некорректное количество: {}", quantity)
            }
            AppError::EmptySheets => write!(f, "Не указаны листы материала"),
            AppError::EmptyParts => write!(f, "Не указаны детали для раскроя"),
            AppError::CalculationError { message } => {
                write!(f, "Ошибка расчета: {}", message)
            }
            AppError::PartsTooBig { part_ids } => {
                write!(f, "Детали слишком большие для листов: {}", part_ids.join(", "))
            }
            AppError::NoSolution => {
                write!(f, "Не удалось найти решение для размещения всех деталей")
            }
            AppError::OptimizationFailed => {
                write!(f, "Ошибка в процессе оптимизации")
            }
            AppError::NotFound { resource, id } => {
                write!(f, "{} с ID '{}' не найден", resource, id)
            }
            AppError::DatabaseError { message } => {
                write!(f, "Ошибка базы данных: {}", message)
            }
            AppError::MigrationError { message } => {
                write!(f, "Ошибка миграции базы данных: {}", message)
            }
            AppError::ExportError { format, message } => {
                write!(f, "Ошибка экспорта в {}: {}", format, message)
            }
            AppError::PdfGenerationFailed => {
                write!(f, "Не удалось сгенерировать PDF")
            }
            AppError::DxfGenerationFailed => {
                write!(f, "Не удалось сгенерировать DXF")
            }
            AppError::ConfigurationError { message } => {
                write!(f, "Ошибка конфигурации: {}", message)
            }
            AppError::EnvironmentError { variable } => {
                write!(f, "Переменная окружения '{}' не установлена", variable)
            }
            AppError::InternalError { message } => {
                write!(f, "Внутренняя ошибка: {}", message)
            }
            AppError::TestError { message } => {
                write!(f, "Тестовая ошибка: {}", message)
            }
        }
    }
}

impl std::error::Error for AppError {}

impl AppError {
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::ValidationError { .. }
            | AppError::InvalidDimensions { .. }
            | AppError::InvalidQuantity { .. }
            | AppError::EmptySheets
            | AppError::EmptyParts => "VALIDATION_ERROR",
            AppError::CalculationError { .. }
            | AppError::PartsTooBig { .. }
            | AppError::NoSolution
            | AppError::OptimizationFailed => "CALCULATION_ERROR",
            AppError::NotFound { .. } => "NOT_FOUND",
            AppError::ExportError { .. }
            | AppError::PdfGenerationFailed
            | AppError::DxfGenerationFailed => "EXPORT_ERROR",
            AppError::DatabaseError { .. }
            | AppError::MigrationError { .. } => "DATABASE_ERROR",
            AppError::ConfigurationError { .. }
            | AppError::EnvironmentError { .. }
            | AppError::InternalError { .. } => "INTERNAL_ERROR",
            AppError::TestError { .. } => "TEST_ERROR",
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::ValidationError { .. }
            | AppError::InvalidDimensions { .. }
            | AppError::InvalidQuantity { .. }
            | AppError::EmptySheets
            | AppError::EmptyParts => StatusCode::BAD_REQUEST,
            AppError::NotFound { .. } => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn details(&self) -> serde_json::Value {
        match self {
            AppError::ValidationError { field, message } => json!({
                "field": field,
                "message": message
            }),
            AppError::InvalidDimensions { width, height } => json!({
                "width": width,
                "height": height
            }),
            AppError::InvalidQuantity { quantity } => json!({
                "quantity": quantity
            }),
            AppError::PartsTooBig { part_ids } => json!({
                "part_ids": part_ids
            }),
            AppError::NotFound { resource, id } => json!({
                "resource": resource,
                "id": id
            }),
            AppError::ExportError { format, message } => json!({
                "format": format,
                "message": message
            }),
            AppError::EnvironmentError { variable } => json!({
                "variable": variable
            }),
            _ => json!({})
        }
    }
}

