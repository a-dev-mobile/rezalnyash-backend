
use std::sync::Arc;
use axum::{Extension, Json, http::StatusCode};
use serde_json::{json, Value};
use tracing::{debug, error, info, warn};

use crate::error::{ApiResult, AppError};
use crate::services::materials::MaterialsService;
use crate::models::materials::MaterialsResponse;
use crate::setting::models::app_state::AppState;

/// Получает список всех доступных материалов
/// 
/// Этот эндпоинт возвращает предустановленные типы материалов 
/// со стандартными размерами, толщинами и свойствами.
/// 
/// # Параметры
/// 
/// * `state` - Состояние приложения с доступом к сервисам
/// 
/// # Возвращает
/// 
/// * `ApiResult<Json<MaterialsResponse>>` - JSON с списком материалов или ошибка
/// 
/// # Коды ответов
/// 
/// * `200 OK` - Успешный запрос со списком материалов
/// * `500 Internal Server Error` - Внутренняя ошибка сервера
/// 
/// # Примеры
/// 
/// ```json
/// {
///   "materials": [
///     {
///       "material_type": "ЛДСП",
///       "name_ru": "ЛДСП (Laminated Chipboard)",
///       "name_en": "Laminated Chipboard",
///       "standard_sizes": [
///         {
///           "width": 2750,
///           "height": 1830,
///           "name": "Стандартный лист",
///           "common_usage": "Мебельное производство"
///         }
///       ],
///       "default_thicknesses": [8, 10, 16, 18, 22, 25, 28, 32],
///       "properties": {
///         "can_rotate": false,
///         "has_grain": true,
///         "recommended_blade_width": 4.0,
///         "recommended_edge_margin": 8.0
///       }
///     }
///   ]
/// }
/// ```
pub async fn get_materials(
    Extension(state): Extension<Arc<AppState>>
) -> ApiResult<Json<MaterialsResponse>> {
    info!("🔍 Запрос списка материалов через API");
    
    // Получаем сервис материалов из состояния приложения
    let materials_service = &state.materials_service;
    
    // Выполняем запрос к сервису
    match materials_service.get_all_materials().await {
        Ok(response) => {
            info!("✅ Успешно получено {} материалов", response.materials.len());
            debug!("Материалы: {:?}", response.materials.iter().map(|m| &m.material_type).collect::<Vec<_>>());
            Ok(Json(response))
        }
        Err(e) => {
            error!("❌ Ошибка при получении материалов: {}", e);
            Err(e)
        }
    }
}

/// Проверяет работоспособность API материалов
/// 
/// Внутренний эндпоинт для проверки состояния компонентов,
/// связанных с материалами.
pub async fn materials_health_check(
    Extension(state): Extension<Arc<AppState>>
) -> ApiResult<Json<Value>> {
    debug!("Проверка работоспособности API материалов");
    
    match state.materials_service.health_check().await {
        Ok(_) => {
            let response = json!({
                "status": "healthy",
                "component": "materials_api",
                "timestamp": chrono::Utc::now().to_rfc3339()
            });
            Ok(Json(response))
        }
        Err(e) => {
            error!("Ошибка при проверке работоспособности materials API: {}", e);
            Err(e)
        }
    }
}