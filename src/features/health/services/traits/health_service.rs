use crate::features::health::{
    domain::errors::HealthError,
    services::dto::HealthStatusDto,
};

#[async_trait::async_trait]
pub trait HealthService: Send + Sync {
    /// Проверить общее здоровье приложения
    async fn check_application_health(&self) -> Result<HealthStatusDto, HealthError>;
    
    /// Проверить здоровье базы данных
    async fn check_database_health(&self) -> Result<HealthStatusDto, HealthError>;
}