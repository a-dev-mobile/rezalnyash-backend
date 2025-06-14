use std::sync::Arc;

use crate::shared::{database::service::PostgresService, setting::models::app_setting::AppSettings};


pub struct AppState {
    pub settings: Arc<AppSettings>,
    pub postgres_service: Arc<PostgresService>,
}

impl AppState {
    pub async fn new(settings: Arc<AppSettings>, postgres_service: Arc<PostgresService>) -> Self {
        // Инициализируем сервис материалов

        Self {
            settings,
            postgres_service,
        }
    }
}
