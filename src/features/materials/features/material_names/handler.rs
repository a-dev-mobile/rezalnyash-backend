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
use super::service::{MaterialNameService, MaterialNameDto, CreateMaterialNameDto};

/// API модели запросов
#[derive(Debug, Deserialize)]
pub struct CreateMaterialNameRequest {
    pub name_ru: String,
    pub name_en: String,
}

/// API модели ответов
#[derive(Debug, Serialize)]
pub struct MaterialNameResponse {
    pub uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

#[derive(Debug, Serialize)]
pub struct MaterialNamesListResponse {
    pub data: Vec<MaterialNameResponse>,
    pub total: usize,
}

impl From<MaterialNameDto> for MaterialNameResponse {
    fn from(dto: MaterialNameDto) -> Self {
        Self {
            uid: dto.uid,
            name_ru: dto.name_ru,
            name_en: dto.name_en,
        }
    }
}

/// Трейт обработчика названий материалов
#[async_trait::async_trait]
pub trait MaterialNameHandler: Send + Sync {
    async fn get_material_name(&self, path: Path<String>) -> Response;
    async fn get_all_material_names(&self) -> Response;
    async fn create_material_name(&self, payload: Json<CreateMaterialNameRequest>) -> Response;
}

/// Реализация обработчика v1
pub struct MaterialNameHandlerV1 {
    service: Arc<dyn MaterialNameService>,
}

impl MaterialNameHandlerV1 {
    pub fn new(service: Arc<dyn MaterialNameService>) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl MaterialNameHandler for MaterialNameHandlerV1 {
    /// GET /api/v1/materials/names/{id}
    async fn get_material_name(&self, Path(id): Path<String>) -> Response {
        let uuid = match Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => {
                return MaterialError::ValidationError {
                    message: format!("Некорректный ID: {}", id),
                }.into_response();
            }
        };

        match self.service.get_material_name(uuid).await {
            Ok(dto) => {
                let response = MaterialNameResponse::from(dto);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    /// GET /api/v1/materials/names
    async fn get_all_material_names(&self) -> Response {
        match self.service.get_all_material_names().await {
            Ok(dtos) => {
                let data: Vec<MaterialNameResponse> = dtos.into_iter().map(MaterialNameResponse::from).collect();
                let response = MaterialNamesListResponse {
                    total: data.len(),
                    data,
                };
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    /// POST /api/v1/materials/names
    async fn create_material_name(&self, Json(payload): Json<CreateMaterialNameRequest>) -> Response {
        let dto = CreateMaterialNameDto {
            name_ru: payload.name_ru,
            name_en: payload.name_en,
        };

        match self.service.create_material_name(dto).await {
            Ok(created_dto) => {
                let response = MaterialNameResponse::from(created_dto);
                (StatusCode::CREATED, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }
}