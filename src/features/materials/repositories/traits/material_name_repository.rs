
use crate::features::materials::domain::{
    entities::MaterialName,
    errors::MaterialError,
    value_objects::MaterialNameUid,
    traits::MaterialNameBehavior,
};

#[async_trait::async_trait]
pub trait MaterialNameRepository: MaterialNameBehavior + Send + Sync {
    /// Проверить подключение к репозиторию
    async fn health_check(&self) -> Result<(), MaterialError>;
}
