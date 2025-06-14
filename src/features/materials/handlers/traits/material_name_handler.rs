
use axum::{
    response::Response,
    extract::Path,
    Json,
};
use crate::features::materials::models::api::v1::{
    CreateMaterialNameRequest,
    MaterialNameResponse,
    MaterialNamesListResponse,
};

#[async_trait::async_trait]
pub trait MaterialNameHandler: Send + Sync {
    /// Получить название материала по ID
    async fn get_material_name(&self, path: Path<String>) -> Response;

    /// Получить все названия материалов
    async fn get_all_material_names(&self) -> Response;

    /// Создать новое название материала
    async fn create_material_name(&self, payload: Json<CreateMaterialNameRequest>) -> Response;
}