// src/features/materials/features/materials/handler.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json as JsonResponse, Response},
};
use std::sync::Arc;
use super::service::MaterialService;

/// Трейт обработчика материалов
#[async_trait::async_trait]
pub trait MaterialHandler: Send + Sync {
    async fn get_all_materials(&self) -> Response;
}

/// Реализация обработчика v1
pub struct MaterialHandlerV1 {
    service: Arc<dyn MaterialService>,
}

impl MaterialHandlerV1 {
    pub fn new(service: Arc<dyn MaterialService>) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl MaterialHandler for MaterialHandlerV1 {
    /// GET /materials
    async fn get_all_materials(&self) -> Response {
        match self.service.get_all_materials().await {
            Ok(materials_list) => {
                (StatusCode::OK, JsonResponse(materials_list)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }
}