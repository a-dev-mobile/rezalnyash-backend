use crate::features::materials::domain::{
    entities::MaterialName, errors::MaterialError, value_objects::MaterialNameId,
};

#[async_trait::async_trait]
pub trait MaterialNameBehavior {
    /// Получить название материала по ID
    async fn get_material_name(&self, id: &MaterialNameId) -> Result<MaterialName, MaterialError>;

    /// Получить все названия материалов
    async fn get_all_material_names(&self) -> Result<Vec<MaterialName>, MaterialError>;

    /// Создать новое название материала
    async fn create_material_name(&self, material_name: MaterialName) -> Result<MaterialName, MaterialError>;

    /// Проверить существование названия материала
    async fn exists(&self, id: &MaterialNameId) -> Result<bool, MaterialError>;

    /// Найти название материала по тексту
    async fn find_by_name(&self, name_ru: &str, name_en: &str) -> Result<Option<MaterialName>, MaterialError>;
}
