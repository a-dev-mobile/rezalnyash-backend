use crate::features::materials::{
    handlers::traits::HeightHandler,
    models::api::v1::{ApiV1Converter, CreateHeightRequest, HeightResponse, HeightsListResponse},
    services::traits::HeightService,
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
    async fn get_height(&self, Path(id): Path<String>) -> Response {
        let uuid = match Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => {
                let error = MaterialError::ValidationError {
                    message: format!("Некорректный формат ID: {}", id),
                };
                return error.into_response();
            }
        };

        match self.service.get_height(uuid).await {
            Ok(dto) => {
                let response = HeightResponse::from_dto(&dto);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    async fn get_all_heights(&self) -> Response {
        match self.service.get_all_heights().await {
            Ok(dtos) => {
                let response = HeightsListResponse::from_dtos(dtos);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    async fn create_height(&self, Json(payload): Json<CreateHeightRequest>) -> Response {
        let dto = ApiV1Converter::create_height_request_to_dto(payload);

        match self.service.create_height(dto).await {
            Ok(created_dto) => {
                let response = HeightResponse::from_dto(&created_dto);
                (StatusCode::CREATED, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }
}