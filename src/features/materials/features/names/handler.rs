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
use super::service::{NameService, NameDto, CreateNameDto};

/// API модели запросов
#[derive(Debug, Deserialize)]
pub struct CreateNameRequest {
    pub name_ru: String,
    pub name_en: String,
}

/// API модели ответов
#[derive(Debug, Serialize)]
pub struct NameResponse {
    pub uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

#[derive(Debug, Serialize)]
pub struct NamesListResponse {
    pub data: Vec<NameResponse>,
    pub total: usize,
}

impl From<NameDto> for NameResponse {
    fn from(dto: NameDto) -> Self {
        Self {
            uid: dto.uid,
            name_ru: dto.name_ru,
            name_en: dto.name_en,
        }
    }
}

/// Трейт обработчика названий материалов
#[async_trait::async_trait]
pub trait NameHandler: Send + Sync {
    async fn get_name(&self, path: Path<String>) -> Response;
    async fn get_all_names(&self) -> Response;
    async fn create_name(&self, payload: Json<CreateNameRequest>) -> Response;
}

/// Реализация обработчика v1
pub struct HandlerV1 {
    service: Arc<dyn NameService>,
}

impl HandlerV1 {
    pub fn new(service: Arc<dyn NameService>) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl NameHandler for HandlerV1 {
    /// GET /api/v1/materials/names/{id}
    async fn get_name(&self, Path(id): Path<String>) -> Response {
        let uuid = match Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => {
                return MaterialError::ValidationError {
                    message: format!("Некорректный ID: {}", id),
                }.into_response();
            }
        };

        match self.service.get_name(uuid).await {
            Ok(dto) => {
                let response = NameResponse::from(dto);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    /// GET /api/v1/materials/names
    async fn get_all_names(&self) -> Response {
        match self.service.get_all_names().await {
            Ok(dtos) => {
                let data: Vec<NameResponse> = dtos.into_iter().map(NameResponse::from).collect();
                let response = NamesListResponse {
                    total: data.len(),
                    data,
                };
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    /// POST /api/v1/materials/names
    async fn create_name(&self, Json(payload): Json<CreateNameRequest>) -> Response {
        let dto = CreateNameDto {
            name_ru: payload.name_ru,
            name_en: payload.name_en,
        };

        match self.service.create_name(dto).await {
            Ok(created_dto) => {
                let response = NameResponse::from(created_dto);
                (StatusCode::CREATED, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }
}