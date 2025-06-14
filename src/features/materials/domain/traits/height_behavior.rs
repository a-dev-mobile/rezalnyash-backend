use crate::features::materials::domain::{
    entities::Height, errors::MaterialError, value_objects::HeightUid,
};

#[async_trait::async_trait]
pub trait HeightBehavior {
    /// Получить высоту по ID
    async fn get_height(&self, id: &HeightUid) -> Result<Height, MaterialError>;

    /// Получить все высоты
    async fn get_all_heights(&self) -> Result<Vec<Height>, MaterialError>;

    /// Создать новую высоту
    async fn create_height(&self, height: Height) -> Result<Height, MaterialError>;

    /// Проверить существование высоты
    async fn exists(&self, id: &HeightUid) -> Result<bool, MaterialError>;

    /// Найти высоту по значению
    async fn find_by_value(&self, value: f64) -> Result<Option<Height>, MaterialError>;
}
