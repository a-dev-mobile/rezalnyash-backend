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
use super::service::{HeightService, HeightDto, CreateHeightDto};

/// API модели запросов
#[derive(Debug, Deserialize)]
pub struct CreateHeightRequest {
    pub height: f64,
}

/// API модели ответов
#[derive(Debug, Serialize)]
pub struct HeightResponse {
    pub uid: Uuid,
    pub height: f64,
}

#[derive(Debug, Serialize)]
pub struct HeightsListResponse {
    pub data: Vec<HeightResponse>,
    pub total: usize,
}

impl From<HeightDto> for HeightResponse {
    fn from(dto: HeightDto) -> Self {
        Self {
            uid: dto.uid,
            height: dto.height,
        }
    }
}

/// Трейт обработчика высот
#[async_trait::async_trait]
pub trait HeightHandler: Send + Sync {
    async fn get_height(&self, path: Path<String>) -> Response;
    async fn get_all_heights(&self) -> Response;
    async fn create_height(&self, payload: Json<CreateHeightRequest>) -> Response;
}

/// Реализация обработчика v1
pub struct HeightHandlerV1 {
    service: Arc<dyn HeightService>,
}

impl HeightHandlerV1 {
    pub fn new(service: Arc<dyn HeightService>) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl HeightHandler for HeightHandlerV1 {
    /// GET /api/v1/materials/heights/{id}
    async fn get_height(&self, Path(id): Path<String>) -> Response {
        let uuid = match Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => {
                return MaterialError::ValidationError {
                    message: format!("Некорректный ID: {}", id),
                }.into_response();
            }
        };

        match self.service.get_height(uuid).await {
            Ok(dto) => {
                let response = HeightResponse::from(dto);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    /// GET /api/v1/materials/heights
    async fn get_all_heights(&self) -> Response {
        match self.service.get_all_heights().await {
            Ok(dtos) => {
                let data: Vec<HeightResponse> = dtos.into_iter().map(HeightResponse::from).collect();
                let response = HeightsListResponse {
                    total: data.len(),
                    data,
                };
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    /// POST /api/v1/materials/heights
    async fn create_height(&self, Json(payload): Json<CreateHeightRequest>) -> Response {
        let dto = CreateHeightDto {
            height: payload.height,
        };

        match self.service.create_height(dto).await {
            Ok(created_dto) => {
                let response = HeightResponse::from(created_dto);
                (StatusCode::CREATED, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }
}