use axum::{extract::Path, response::Response, Json};
use crate::features::materials::models::api::v1::{
    CreateWidthRequest, WidthResponse, WidthsListResponse,
};

#[async_trait::async_trait]
pub trait WidthHandler: Send + Sync {
    async fn get_width(&self, path: Path<String>) -> Response;
    async fn get_all_widths(&self) -> Response;
    async fn create_width(&self, payload: Json<CreateWidthRequest>) -> Response;
}