use sqlx::PgPool;
use crate::features::health::{
    domain::{
        entities::HealthStatus,
        errors::HealthError,
        traits::HealthBehavior,
    },
    repositories::traits::HealthRepository,
};

pub struct PostgresHealthRepository {
    pool: PgPool,
}

impl PostgresHealthRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl HealthBehavior for PostgresHealthRepository {
    async fn check_application_health(&self) -> Result<HealthStatus, HealthError> {
        Ok(HealthStatus::healthy("application".to_string())
            .with_message("Application is running".to_string()))
    }

    async fn check_database_health(&self) -> Result<HealthStatus, HealthError> {
        match sqlx::query("SELECT 1").fetch_one(&self.pool).await {
            Ok(_) => Ok(HealthStatus::healthy("database".to_string())
                .with_message("Database connection successful".to_string())),
            Err(e) => Err(HealthError::DatabaseHealthCheckFailed {
                message: e.to_string(),
            }),
        }
    }
}

#[async_trait::async_trait]
impl HealthRepository for PostgresHealthRepository {
    async fn health_check(&self) -> Result<(), HealthError> {
        self.check_database_health().await?;
        Ok(())
    }
}