use std::sync::Arc;
use crate::features::health::{
    domain::errors::HealthError,
    repositories::traits::HealthRepository,
    services::{
        dto::HealthStatusDto,
        traits::HealthService,
    },
};

pub struct HealthServiceImpl {
    repository: Arc<dyn HealthRepository>,
}

impl HealthServiceImpl {
    pub fn new(repository: Arc<dyn HealthRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl HealthService for HealthServiceImpl {
    async fn check_application_health(&self) -> Result<HealthStatusDto, HealthError> {
        let status = self.repository.check_application_health().await?;
        Ok(HealthStatusDto::from_domain(&status))
    }

    async fn check_database_health(&self) -> Result<HealthStatusDto, HealthError> {
        let status = self.repository.check_database_health().await?;
        Ok(HealthStatusDto::from_domain(&status))
    }
}