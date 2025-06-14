use crate::features::health::domain::{
    entities::HealthStatus,
    errors::HealthError,
    traits::HealthBehavior,
};

#[async_trait::async_trait]
pub trait HealthRepository: HealthBehavior + Send + Sync {
    /// Проверить подключение к репозиторию
    async fn health_check(&self) -> Result<(), HealthError>;
}