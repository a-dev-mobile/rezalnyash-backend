use crate::features::materials::domain::{
    entities::MaterialType,
    errors::MaterialError,
    value_objects::MaterialTypeId,
    traits::MaterialTypeBehavior,
};

#[async_trait::async_trait]
pub trait MaterialTypeRepository: MaterialTypeBehavior + Send + Sync {
    /// Проверить подключение к репозиторию
    async fn health_check(&self) -> Result<(), MaterialError>;
}