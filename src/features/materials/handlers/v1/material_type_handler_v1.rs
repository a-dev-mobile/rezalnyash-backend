use crate::features::materials::{
    handlers::traits::MaterialTypeHandler,
    models::api::v1::{ApiV1Converter, CreateMaterialTypeRequest, MaterialTypeResponse, MaterialTypesListResponse},
    services::traits::MaterialTypeService,
    MaterialError,
};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json as JsonResponse, Response},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct MaterialTypeHandlerV1 {
    service: Arc<dyn MaterialTypeService>,
}

impl MaterialTypeHandlerV1 {

    pub fn new(service: Arc<dyn MaterialTypeService>) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl MaterialTypeHandler for MaterialTypeHandlerV1 {
  
    async fn get_material_type(&self, Path(id): Path<String>) -> Response {
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
        match self.service.get_material_type(uuid).await {
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


    async fn get_all_material_types(&self) -> Response {
        match self.service.get_all_material_types().await {
            Ok(dtos) => {
                let response = MaterialTypesListResponse::from_dtos(dtos);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

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
