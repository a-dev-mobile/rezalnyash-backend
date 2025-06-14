use crate::features::materials::domain::{
    entities::MaterialType, errors::MaterialError, value_objects::MaterialTypeUid,
};

#[async_trait::async_trait]
pub trait MaterialTypeBehavior {
    /// Получить тип материала по ID
    async fn get_material_type(&self, id: &MaterialTypeUid) -> Result<MaterialType, MaterialError>;

    /// Получить все типы материалов
    async fn get_all_material_types(&self) -> Result<Vec<MaterialType>, MaterialError>;

    /// Создать новый тип материала
    async fn create_material_type(&self, material_type: MaterialType) -> Result<MaterialType, MaterialError>;

    /// Проверить существование типа материала
    async fn exists(&self, id: &MaterialTypeUid) -> Result<bool, MaterialError>;

    /// Найти тип материала по названию
    async fn find_by_name(&self, name_ru: &str, name_en: &str) -> Result<Option<MaterialType>, MaterialError>;
}
