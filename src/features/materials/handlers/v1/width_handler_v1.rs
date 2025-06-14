use crate::features::materials::{
    handlers::traits::WidthHandler,
    models::api::v1::{ApiV1Converter, CreateWidthRequest, WidthResponse, WidthsListResponse},
    services::traits::WidthService,
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
    async fn get_width(&self, Path(id): Path<String>) -> Response {
        let uuid = match Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => {
                let error = MaterialError::ValidationError {
                    message: format!("Некорректный формат ID: {}", id),
                };
                return error.into_response();
            }
        };

        match self.service.get_width(uuid).await {
            Ok(dto) => {
                let response = WidthResponse::from_dto(&dto);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    async fn get_all_widths(&self) -> Response {
        match self.service.get_all_widths().await {
            Ok(dtos) => {
                let response = WidthsListResponse::from_dtos(dtos);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    async fn create_width(&self, Json(payload): Json<CreateWidthRequest>) -> Response {
        let dto = ApiV1Converter::create_width_request_to_dto(payload);

        match self.service.create_width(dto).await {
            Ok(created_dto) => {
                let response = WidthResponse::from_dto(&created_dto);
                (StatusCode::CREATED, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }
}
