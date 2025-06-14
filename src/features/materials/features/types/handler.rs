use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json as JsonResponse, Response},
    Json,
};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::features::materials::shared::errors::MaterialError;
use super::service::{TypeService, MaterialTypeDto, CreateMaterialTypeDto};

/// API модели запросов
#[derive(Debug, Deserialize)]
pub struct CreateMaterialTypeRequest {
    pub name_ru: String,
    pub name_en: String,
}

/// API модели ответов
#[derive(Debug, Serialize)]
pub struct MaterialTypeResponse {
    pub uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

#[derive(Debug, Serialize)]
pub struct MaterialTypesListResponse {
    pub data: Vec<MaterialTypeResponse>,
    pub total: usize,
}

impl From<MaterialTypeDto> for MaterialTypeResponse {
    fn from(dto: MaterialTypeDto) -> Self {
        Self {
            uid: dto.uid,
            name_ru: dto.name_ru,
            name_en: dto.name_en,
        }
    }
}

/// Трейт обработчика типов материалов
#[async_trait::async_trait]
pub trait TypeHandler: Send + Sync {
    async fn get_type(&self, path: Path<String>) -> Response;
    async fn get_all_types(&self) -> Response;
    async fn create_type(&self, payload: Json<CreateMaterialTypeRequest>) -> Response;
}

/// Реализация обработчика v1
pub struct MaterialTypeHandlerV1 {
    service: Arc<dyn TypeService>,
}

impl MaterialTypeHandlerV1 {
    pub fn new(service: Arc<dyn TypeService>) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl TypeHandler for MaterialTypeHandlerV1 {
    /// GET /api/v1/materials/types/{id}
    async fn get_type(&self, Path(id): Path<String>) -> Response {
        let uuid = match Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => {
                return MaterialError::ValidationError {
                    message: format!("Некорректный ID: {}", id),
                }.into_response();
            }
        };

        match self.service.get_type(uuid).await {
            Ok(dto) => {
                let response = MaterialTypeResponse::from(dto);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    /// GET /api/v1/materials/types
    async fn get_all_types(&self) -> Response {
        match self.service.get_all_types().await {
            Ok(dtos) => {
                let data: Vec<MaterialTypeResponse> = dtos.into_iter().map(MaterialTypeResponse::from).collect();
                let response = MaterialTypesListResponse {
                    total: data.len(),
                    data,
                };
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    /// POST /api/v1/materials/types
    async fn create_type(&self, Json(payload): Json<CreateMaterialTypeRequest>) -> Response {
        let dto = CreateMaterialTypeDto {
            name_ru: payload.name_ru,
            name_en: payload.name_en,
        };

        match self.service.create_type(dto).await {
            Ok(created_dto) => {
                let response = MaterialTypeResponse::from(created_dto);
                (StatusCode::CREATED, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }
}