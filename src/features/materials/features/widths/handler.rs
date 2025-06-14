// widths/handler.rs - HTTP обработчик
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

use super::service::{WidthService, WidthDto, CreateWidthDto};

/// API модели запросов
#[derive(Debug, Deserialize)]
pub struct CreateWidthRequest {
    pub width: f64,
}

/// API модели ответов
#[derive(Debug, Serialize)]
pub struct WidthResponse {
    pub uid: Uuid,
    pub width: f64,
}

#[derive(Debug, Serialize)]
pub struct WidthsListResponse {
    pub data: Vec<WidthResponse>,
    pub total: usize,
}

impl From<WidthDto> for WidthResponse {
    fn from(dto: WidthDto) -> Self {
        Self {
            uid: dto.uid,
            width: dto.width,
        }
    }
}

/// Трейт обработчика ширин
#[async_trait::async_trait]
pub trait WidthHandler: Send + Sync {
    async fn get_width(&self, path: Path<String>) -> Response;
    async fn get_all_widths(&self) -> Response;
    async fn create_width(&self, payload: Json<CreateWidthRequest>) -> Response;
}

/// Реализация обработчика v1
pub struct WidthHandlerV1 {
    service: Arc<dyn WidthService>,
}

impl WidthHandlerV1 {
    pub fn new(service: Arc<dyn WidthService>) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl WidthHandler for WidthHandlerV1 {
    /// GET /widths/{id}
    async fn get_width(&self, Path(id): Path<String>) -> Response {
        let uuid = match Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => {
                return MaterialError::ValidationError {
                    message: format!("Некорректный ID: {}", id),
                }.into_response();
            }
        };

        match self.service.get_width(uuid).await {
            Ok(dto) => {
                let response = WidthResponse::from(dto);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    /// GET /widths
    async fn get_all_widths(&self) -> Response {
        match self.service.get_all_widths().await {
            Ok(dtos) => {
                let data: Vec<WidthResponse> = dtos.into_iter().map(WidthResponse::from).collect();
                let response = WidthsListResponse {
                    total: data.len(),
                    data,
                };
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    /// POST /widths
    async fn create_width(&self, Json(payload): Json<CreateWidthRequest>) -> Response {
        let dto = CreateWidthDto {
            width: payload.width,
        };

        match self.service.create_width(dto).await {
            Ok(created_dto) => {
                let response = WidthResponse::from(created_dto);
                (StatusCode::CREATED, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }
}
