use uuid::Uuid;

use crate::features::materials::{
    domain::errors::MaterialError,
    services::dto::{CreateMaterialTypeDto, MaterialTypeDto},
};

#[async_trait::async_trait]
pub trait MaterialTypeService: Send + Sync {
    /// Получить тип материала по ID
    async fn get_material_type(&self, id: Uuid) -> Result<MaterialTypeDto, MaterialError>;

    /// Получить все типы материалов
    async fn get_all_material_types(&self) -> Result<Vec<MaterialTypeDto>, MaterialError>;

    /// Создать новый тип материала
    async fn create_material_type(&self, dto: CreateMaterialTypeDto) -> Result<MaterialTypeDto, MaterialError>;

    /// Проверить существует ли тип материала
    async fn exists(&self, id: Uuid) -> Result<bool, MaterialError>;
}
