use sqlx::PgPool;

use crate::features::materials::shared::errors::MaterialError;


/// Проверка подключения к БД
pub async fn check_database_connection(pool: &PgPool) -> Result<(), MaterialError> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Нет подключения к БД: {}", e),
        })?;
    Ok(())
}


/// Общие константы для БД
pub mod db_constants {
    pub const MATERIALS_SCHEMA: &str = "materials";
    pub const MAX_STRING_LENGTH: usize = 255;
    pub const MAX_NUMERIC_PRECISION: u32 = 10;
    pub const MAX_NUMERIC_SCALE: u32 = 2;
}