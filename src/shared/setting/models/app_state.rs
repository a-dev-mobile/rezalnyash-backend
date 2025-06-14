use std::sync::Arc;

use crate::{
    features::materials::features::widths::{
        handler::{WidthHandler, WidthHandlerV1}, repository::{PostgresWidthRepository, WidthRepository}, service::{WidthService, WidthServiceImpl},
    },
    shared::{database::service::PostgresService, setting::models::app_setting::AppSettings},
};

pub struct AppState {
    pub settings: Arc<AppSettings>,
    pub postgres_service: Arc<PostgresService>,
    // Width feature dependencies
    pub width_handler: Arc<dyn WidthHandler>,
    pub width_service: Arc<dyn WidthService>,
    pub width_repository: Arc<dyn WidthRepository>,
}

impl AppState {
    pub async fn new(settings: Arc<AppSettings>, postgres_service: Arc<PostgresService>) -> Self {
        // Получаем pool из postgres_service
        let pool = postgres_service.connection.pool().clone();

        // Создаем зависимости для widths feature
        let width_repository: Arc<dyn WidthRepository> = Arc::new(PostgresWidthRepository::new(pool.clone()));
        let width_service: Arc<dyn WidthService> = Arc::new(WidthServiceImpl::new(width_repository.clone()));
        let width_handler: Arc<dyn WidthHandler> = Arc::new(WidthHandlerV1::new(width_service.clone()));

        Self {
            settings,
            postgres_service,
            width_handler,
            width_service,
            width_repository,
        }
    }
}
