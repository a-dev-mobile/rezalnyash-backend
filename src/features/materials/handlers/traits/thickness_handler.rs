use axum::{extract::Path, response::Response, Json};
use crate::features::materials::models::api::v1::{
    CreateThicknessRequest, ThicknessResponse, ThicknessesListResponse,
};

#[async_trait::async_trait]
pub trait ThicknessHandler: Send + Sync {
    async fn get_thickness(&self, path: Path<String>) -> Response;
    async fn get_all_thicknesses(&self) -> Response;
    async fn create_thickness(&self, payload: Json<CreateThicknessRequest>) -> Response;
}