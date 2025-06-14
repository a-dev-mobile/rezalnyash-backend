use crate::features::materials::models::api::v1::{
    CreateMaterialTypeRequest, MaterialTypeResponse, MaterialTypesListResponse,
};
use axum::{extract::Path, response::Response, Json};

#[async_trait::async_trait]
pub trait MaterialTypeHandler: Send + Sync {
    /// Получить тип материала по ID
    async fn get_material_type(&self, path: Path<String>) -> Response;

    /// Получить все типы материалов
    async fn get_all_material_types(&self) -> Response;

    /// Создать новый тип материала
    async fn create_material_type(&self, payload: Json<CreateMaterialTypeRequest>) -> Response;
}
