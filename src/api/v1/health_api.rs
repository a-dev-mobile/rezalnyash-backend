use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use serde_json::{json, Value};

use crate::{error::{ApiResult, AppError}, setting::models::app_state::AppState};


pub async fn health_api() -> ApiResult<Json<Value>> {
    let result = json!({
            "status": "healthy",
    });

    Ok(Json(result))

    // Err(AppError::InternalError { message:  "111test error".to_string(),
    // })

    
}
// Новый endpoint для проверки здоровья БД
pub async fn health_db(Extension(state): Extension<Arc<AppState>>) -> ApiResult<Json<Value>> {
    // Проверяем подключение к БД - если есть ошибка, она автоматически конвертируется в AppError
    state.postgres_service.connection.health_check().await?;
    
    let result = json!({
        "status": "healthy",
        "database": "connected"
    });

    Ok(Json(result))
}

// Пример функции которая может генерировать DatabaseError
pub async fn test_db_error(Extension(state): Extension<Arc<AppState>>) -> ApiResult<Json<Value>> {
    // Специально делаем невалидный запрос для демонстрации ошибки
    let _result: Vec<(i32,)> = sqlx::query_as("SELECT * FROM lo2cal.users")
        .fetch_all(state.postgres_service.connection.pool())
        .await?; // Автоматическая конверсия sqlx::Error -> AppError -> HTTP response
    
    Ok(Json(json!({"status": "ok"})))
}