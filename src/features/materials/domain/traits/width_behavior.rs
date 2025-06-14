use crate::features::materials::domain::{
    entities::Width, errors::MaterialError, value_objects::WidthUid,
};

#[async_trait::async_trait]
pub trait WidthBehavior {
    /// Получить ширину по ID
    async fn get_width(&self, id: &WidthUid) -> Result<Width, MaterialError>;

    /// Получить все ширины
    async fn get_all_widths(&self) -> Result<Vec<Width>, MaterialError>;

    /// Создать новую ширину
    async fn create_width(&self, width: Width) -> Result<Width, MaterialError>;

    /// Проверить существование ширины
    async fn exists(&self, id: &WidthUid) -> Result<bool, MaterialError>;

    /// Найти ширину по значению
    async fn find_by_value(&self, value: f64) -> Result<Option<Width>, MaterialError>;
}
