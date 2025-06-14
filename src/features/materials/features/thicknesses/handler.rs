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
use super::service::{ThicknessService, ThicknessDto, CreateThicknessDto};

/// API модели запросов
#[derive(Debug, Deserialize)]
pub struct CreateThicknessRequest {
    pub thickness: f64,
}

/// API модели ответов
#[derive(Debug, Serialize)]
pub struct ThicknessResponse {
    pub uid: Uuid,
    pub thickness: f64,
}

#[derive(Debug, Serialize)]
pub struct ThicknessesListResponse {
    pub data: Vec<ThicknessResponse>,
    pub total: usize,
}

impl From<ThicknessDto> for ThicknessResponse {
    fn from(dto: ThicknessDto) -> Self {
        Self {
            uid: dto.uid,
            thickness: dto.thickness,
        }
    }
}

/// Трейт обработчика толщин
#[async_trait::async_trait]
pub trait ThicknessHandler: Send + Sync {
    async fn get_thickness(&self, path: Path<String>) -> Response;
    async fn get_all_thicknesses(&self) -> Response;
    async fn create_thickness(&self, payload: Json<CreateThicknessRequest>) -> Response;
}

/// Реализация обработчика v1
pub struct ThicknessHandlerV1 {
    service: Arc<dyn ThicknessService>,
}

impl ThicknessHandlerV1 {
    pub fn new(service: Arc<dyn ThicknessService>) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl ThicknessHandler for ThicknessHandlerV1 {
    /// GET /api/v1/materials/thicknesses/{id}
    async fn get_thickness(&self, Path(id): Path<String>) -> Response {
        let uuid = match Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => {
                return MaterialError::ValidationError {
                    message: format!("Некорректный ID: {}", id),
                }.into_response();
            }
        };

        match self.service.get_thickness(uuid).await {
            Ok(dto) => {
                let response = ThicknessResponse::from(dto);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    /// GET /api/v1/materials/thicknesses
    async fn get_all_thicknesses(&self) -> Response {
        match self.service.get_all_thicknesses().await {
            Ok(dtos) => {
                let data: Vec<ThicknessResponse> = dtos.into_iter().map(ThicknessResponse::from).collect();
                let response = ThicknessesListResponse {
                    total: data.len(),
                    data,
                };
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    /// POST /api/v1/materials/thicknesses
    async fn create_thickness(&self, Json(payload): Json<CreateThicknessRequest>) -> Response {
        let dto = CreateThicknessDto {
            thickness: payload.thickness,
        };

        match self.service.create_thickness(dto).await {
            Ok(created_dto) => {
                let response = ThicknessResponse::from(created_dto);
                (StatusCode::CREATED, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }
}