use axum::{extract::Path, response::Response, Json};
use crate::features::materials::models::api::v1::{
    CreateHeightRequest, HeightResponse, HeightsListResponse,
};

#[async_trait::async_trait]
pub trait HeightHandler: Send + Sync {
    async fn get_height(&self, path: Path<String>) -> Response;
    async fn get_all_heights(&self) -> Response;
    async fn create_height(&self, payload: Json<CreateHeightRequest>) -> Response;
}