use crate::features::health::{
    handlers::{traits::HealthHandler, v1::HealthHandlerV1},
    repositories::{implementations::PostgresHealthRepository, traits::HealthRepository},
    routes::v1::health_routes_v1,
    services::{implementations::HealthServiceImpl, traits::HealthService},
};
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

pub struct HealthRoutesBuilder;

impl HealthRoutesBuilder {
    /// Создает полный набор маршрутов для health v1
    pub fn build_v1(pool: PgPool) -> Router {
        // Создаем репозиторий
        let health_repo: Arc<dyn HealthRepository> = Arc::new(PostgresHealthRepository::new(pool));

        // Создаем сервис
        let health_service: Arc<dyn HealthService> = Arc::new(HealthServiceImpl::new(health_repo));

        // Создаем handler
        let health_handler: Arc<dyn HealthHandler> = Arc::new(HealthHandlerV1::new(health_service));

        // Возвращаем маршруты
        health_routes_v1(health_handler)
    }
}