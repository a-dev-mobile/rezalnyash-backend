use super::types::AppError;

// Только системные конверсии
impl From<sqlx::migrate::MigrateError> for AppError {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        AppError::MigrationError {
            message: err.to_string(),
        }
    }
}

// Базовая конверсия для критичных ошибок подключения
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseConnectionError {
            message: err.to_string(),
        }
    }
}