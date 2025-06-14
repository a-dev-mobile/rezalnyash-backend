use crate::features::materials::{
    handlers::traits::ThicknessHandler,
    models::api::v1::{ApiV1Converter, CreateThicknessRequest, ThicknessResponse, ThicknessesListResponse},
    services::traits::ThicknessService,
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
    async fn get_thickness(&self, Path(id): Path<String>) -> Response {
        let uuid = match Uuid::parse_str(&id) {
            Ok(uuid) => uuid,
            Err(_) => {
                let error = MaterialError::ValidationError {
                    message: format!("Некорректный формат ID: {}", id),
                };
                return error.into_response();
            }
        };

        match self.service.get_thickness(uuid).await {
            Ok(dto) => {
                let response = ThicknessResponse::from_dto(&dto);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    async fn get_all_thicknesses(&self) -> Response {
        match self.service.get_all_thicknesses().await {
            Ok(dtos) => {
                let response = ThicknessesListResponse::from_dtos(dtos);
                (StatusCode::OK, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }

    async fn create_thickness(&self, Json(payload): Json<CreateThicknessRequest>) -> Response {
        let dto = ApiV1Converter::create_thickness_request_to_dto(payload);

        match self.service.create_thickness(dto).await {
            Ok(created_dto) => {
                let response = ThicknessResponse::from_dto(&created_dto);
                (StatusCode::CREATED, JsonResponse(response)).into_response()
            }
            Err(error) => error.into_response(),
        }
    }
}