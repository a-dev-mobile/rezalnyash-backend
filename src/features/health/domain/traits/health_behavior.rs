use crate::features::health::domain::{entities::HealthStatus, errors::HealthError};

#[async_trait::async_trait]
pub trait HealthBehavior {
    /// Проверить общее здоровье приложения
    async fn check_application_health(&self) -> Result<HealthStatus, HealthError>;

    /// Проверить здоровье базы данных
    async fn check_database_health(&self) -> Result<HealthStatus, HealthError>;
}
