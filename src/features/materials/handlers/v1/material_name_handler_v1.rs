use crate::features::materials::{
    handlers::traits::MaterialNameHandler,
    models::api::v1::{ApiV1Converter, CreateMaterialNameRequest, MaterialNameResponse, MaterialNamesListResponse},
    services::traits::MaterialNameService, MaterialError,
};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json as JsonResponse, Response},
    Json,
};
use uuid::Uuid;
use std::sync::Arc;

pub struct MaterialNameHandlerV1 {
    service: Arc<dyn MaterialNameService>,
}

impl MaterialNameHandlerV1 {
    /// Создает новый экземпляр обработчика
    ///
    /// # Arguments
    /// * `service` - Сервис для работы с названиями материалов
    pub fn new(service: Arc<dyn MaterialNameService>) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl MaterialNameHandler for MaterialNameHandlerV1 {
    async fn get_material_name(&self, Path(id): Path<String>) -> Response {
        // Парсим UUID из строки
        let uuid = match Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => {
                let error = MaterialError::ValidationError {
                    message: format!("Некорректный формат ID: {}", id),
                };
                return error.into_response();
            }
        };
        match self.service.get_material_name(uuid).await {
            Ok(dto) => {
                let response = MaterialNameResponse::from_dto(&dto);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => {
                error.into_response()
            }
        }
    }

    /// Получить все названия материалов
    ///
    /// # Endpoint
    /// GET /api/v1/materials/names
    ///
    /// # Responses
    /// - 200 OK: Список названий материалов (может быть пустым)
    /// - 500 Internal Server Error: Ошибка сервера или базы данных
    ///
    /// # Example Success Response
    /// ```json
    /// {
    ///   "data": [
    ///     {
    ///       "id": 1,
    ///       "name_ru": "Сосна обрезная",
    ///       "name_en": "Pine lumber"
    ///     },
    ///     {
    ///       "id": 2,
    ///       "name_ru": "Дуб массив",
    ///       "name_en": "Oak solid wood"
    ///     }
    ///   ],
    ///   "total": 2
    /// }
    /// ```
    async fn get_all_material_names(&self) -> Response {
        match self.service.get_all_material_names().await {
            Ok(dtos) => {
                let response = MaterialNamesListResponse::from_dtos(dtos);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    /// Создать новое название материала
    ///
    /// # Endpoint
    /// POST /api/v1/materials/names
    ///
    /// # Request Body
    /// ```json
    /// {
    ///   "name_ru": "Сосна обрезная",
    ///   "name_en": "Pine lumber"
    /// }
    /// ```
    ///
    /// # Responses
    /// - 201 Created: Название материала успешно создано
    /// - 400 Bad Request: Ошибка валидации (пустые названия)
    /// - 409 Conflict: Название материала с такими названиями уже существует
    /// - 500 Internal Server Error: Ошибка сервера или базы данных
    ///
    /// # Example Success Response
    /// ```json
    /// {
    ///   "id": 3,
    ///   "name_ru": "Сосна обрезная",
    ///   "name_en": "Pine lumber"
    /// }
    /// ```
    ///
    /// # Example Validation Error Response
    /// ```json
    /// {
    ///   "code": "MATERIAL_VALIDATION_ERROR",
    ///   "message": "Название материала не может быть пустым (русский)",
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
    ///   "message": "Название материала 'Сосна обрезная' / 'Pine lumber' уже существует",
    ///   "details": {
    ///     "name_ru": "Сосна обрезная",
    ///     "name_en": "Pine lumber",
    ///     "resource": "material_name"
    ///   },
    ///   "timestamp": "2025-06-14T15:30:45.123Z"
    /// }
    /// ```
    async fn create_material_name(&self, Json(payload): Json<CreateMaterialNameRequest>) -> Response {
        // Конвертируем API запрос в DTO для сервиса
        let dto = ApiV1Converter::create_material_name_request_to_dto(payload);

        match self.service.create_material_name(dto).await {
            Ok(created_dto) => {
                let response = MaterialNameResponse::from_dto(&created_dto);
                (StatusCode::CREATED, JsonResponse(response)).into_response()
            }
            Err(error) => {
                // Система ошибок автоматически определит правильный HTTP статус:
                // - MaterialNameEmpty -> 400 Bad Request
                // - MaterialNameDuplicate -> 409 Conflict
                // - DatabaseError -> 500 Internal Server Error
                error.into_response()
            }
        }
    }
}
