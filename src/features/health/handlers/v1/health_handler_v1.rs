use crate::features::health::{
    handlers::traits::HealthHandler,
    models::api::v1::HealthResponse,
    services::traits::HealthService,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json as JsonResponse, Response},
};
use std::sync::Arc;

pub struct HealthHandlerV1 {
    service: Arc<dyn HealthService>,
}

impl HealthHandlerV1 {
    pub fn new(service: Arc<dyn HealthService>) -> Self {
        Self { service }
    }
}

#[async_trait::async_trait]
impl HealthHandler for HealthHandlerV1 {
    async fn check_application_health(&self) -> Response {
        match self.service.check_application_health().await {
            Ok(dto) => {
                let response = HealthResponse::from_dto(&dto);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    async fn check_database_health(&self) -> Response {
        match self.service.check_database_health().await {
            Ok(dto) => {
                let response = HealthResponse::from_dto(&dto);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }
}