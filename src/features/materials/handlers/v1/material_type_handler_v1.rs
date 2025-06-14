
use std::sync::Arc;
use axum::{
    extract::Path, http::StatusCode, response::{IntoResponse, Json as JsonResponse, Response}, Json
};
use crate::features::materials::{
    handlers::traits::MaterialTypeHandler,
    services::traits::MaterialTypeService,
    models::api::v1::{
        CreateMaterialTypeRequest,
        MaterialTypeResponse,
        MaterialTypesListResponse,
        ApiV1Converter,
    },
};

/// HTTP обработчик для типов материалов версии 1
/// 
/// Этот обработчик отвечает за:
/// - Получение типа материала по ID
/// - Получение списка всех типов материалов
/// - Создание нового типа материала
/// 
/// Все ошибки автоматически преобразуются в соответствующие HTTP ответы
/// благодаря реализации IntoResponse для MaterialError
pub struct MaterialTypeHandlerV1 {
    service: Arc<dyn MaterialTypeService>,
}

impl MaterialTypeHandlerV1 {
    /// Создает новый экземпляр обработчика
    /// 
    /// # Arguments
    /// * `service` - Сервис для работы с типами материалов
    pub fn new(service: Arc<dyn MaterialTypeService>) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl MaterialTypeHandler for MaterialTypeHandlerV1 {
    /// Получить тип материала по ID
    /// 
    /// # Endpoint
    /// GET /api/v1/materials/types/{id}
    /// 
    /// # Responses
    /// - 200 OK: Тип материала найден и возвращен
    /// - 400 Bad Request: Некорректный ID (например, отрицательный)
    /// - 404 Not Found: Тип материала с указанным ID не найден
    /// - 500 Internal Server Error: Ошибка сервера или базы данных
    /// 
    /// # Example Success Response
    /// ```json
    /// {
    ///   "id": 1,
    ///   "name_ru": "Древесина",
    ///   "name_en": "Wood"
    /// }
    /// ```
    /// 
    /// # Example Error Response
    /// ```json
    /// {
    ///   "code": "MATERIAL_NOT_FOUND",
    ///   "message": "Тип материала с ID 999 не найден",
    ///   "details": {
    ///     "id": 999,
    ///     "resource": "material_type"
    ///   },
    ///   "timestamp": "2025-06-14T15:30:45.123Z"
    /// }
    /// ```
    async fn get_material_type(&self, Path(id): Path<i32>) -> Response {
        match self.service.get_material_type(id).await {
            Ok(dto) => {
                let response = MaterialTypeResponse::from_dto(&dto);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => {
                // MaterialError автоматически преобразуется в правильный HTTP ответ
                // с соответствующим статус кодом и структурированной ошибкой
                error.into_response()
            }
        }
    }

    /// Получить все типы материалов
    /// 
    /// # Endpoint
    /// GET /api/v1/materials/types
    /// 
    /// # Responses
    /// - 200 OK: Список типов материалов (может быть пустым)
    /// - 500 Internal Server Error: Ошибка сервера или базы данных
    /// 
    /// # Example Success Response
    /// ```json
    /// {
    ///   "data": [
    ///     {
    ///       "id": 1,
    ///       "name_ru": "Древесина",
    ///       "name_en": "Wood"
    ///     },
    ///     {
    ///       "id": 2,
    ///       "name_ru": "Фанера",
    ///       "name_en": "Plywood"
    ///     }
    ///   ],
    ///   "total": 2
    /// }
    /// ```
    async fn get_all_material_types(&self) -> Response {
        match self.service.get_all_material_types().await {
            Ok(dtos) => {
                let response = MaterialTypesListResponse::from_dtos(dtos);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => {
                error.into_response()
            }
        }
    }

    /// Создать новый тип материала
    /// 
    /// # Endpoint
    /// POST /api/v1/materials/types
    /// 
    /// # Request Body
    /// ```json
    /// {
    ///   "name_ru": "Древесина",
    ///   "name_en": "Wood"
    /// }
    /// ```
    /// 
    /// # Responses
    /// - 201 Created: Тип материала успешно создан
    /// - 400 Bad Request: Ошибка валидации (пустые названия)
    /// - 409 Conflict: Тип материала с такими названиями уже существует
    /// - 500 Internal Server Error: Ошибка сервера или базы данных
    /// 
    /// # Example Success Response
    /// ```json
    /// {
    ///   "id": 3,
    ///   "name_ru": "Древесина",
    ///   "name_en": "Wood"
    /// }
    /// ```
    /// 
    /// # Example Validation Error Response
    /// ```json
    /// {
    ///   "code": "MATERIAL_VALIDATION_ERROR",
    ///   "message": "Название типа материала не может быть пустым (русский)",
    ///   "details": {
    ///     "language": "русский"
    ///   },
    ///   "timestamp": "2025-06-14T15:30:45.123Z"
    /// }
    /// ```
    /// 
    /// # Example Duplicate Error Response
    /// ```json
    /// {
    ///   "code": "MATERIAL_DUPLICATE",
    ///   "message": "Тип материала с названиями 'Древесина' / 'Wood' уже существует",
    ///   "details": {
    ///     "name_ru": "Древесина",
    ///     "name_en": "Wood",
    ///     "resource": "material_type"
    ///   },
    ///   "timestamp": "2025-06-14T15:30:45.123Z"
    /// }
    /// ```
    async fn create_material_type(&self, Json(payload): Json<CreateMaterialTypeRequest>) -> Response {
        // Конвертируем API запрос в DTO для сервиса
        let dto = ApiV1Converter::create_material_type_request_to_dto(payload);
        
        match self.service.create_material_type(dto).await {
            Ok(created_dto) => {
                let response = MaterialTypeResponse::from_dto(&created_dto);
                (StatusCode::CREATED, JsonResponse(response)).into_response()
            }
            Err(error) => {
                // Система ошибок автоматически определит правильный HTTP статус:
                // - MaterialTypeNameEmpty -> 400 Bad Request
                // - MaterialTypeDuplicate -> 409 Conflict
                // - DatabaseError -> 500 Internal Server Error
                error.into_response()
            }
        }
    }
}