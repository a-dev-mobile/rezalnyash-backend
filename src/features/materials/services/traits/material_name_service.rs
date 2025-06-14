
use uuid::Uuid;

use crate::features::materials::{
    domain::errors::MaterialError,
    services::dto::{CreateMaterialNameDto, MaterialNameDto},
};

#[async_trait::async_trait]
pub trait MaterialNameService: Send + Sync {
    /// Получить название материала по ID
    async fn get_material_name(&self, id: Uuid) -> Result<MaterialNameDto, MaterialError>;

    /// Получить все названия материалов
    async fn get_all_material_names(&self) -> Result<Vec<MaterialNameDto>, MaterialError>;

    /// Создать новое название материала
    async fn create_material_name(&self, dto: CreateMaterialNameDto) -> Result<MaterialNameDto, MaterialError>;

    /// Проверить существует ли название материала
    async fn exists(&self, id: Uuid) -> Result<bool, MaterialError>;
}