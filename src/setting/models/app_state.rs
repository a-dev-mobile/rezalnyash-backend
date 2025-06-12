use std::sync::Arc;

use crate::{database::{repositories::MaterialsRepository, service::PostgresService}, services::materials::MaterialsService, setting::models::app_setting::AppSettings};

pub struct AppState {
    pub settings: Arc<AppSettings>,
    pub postgres_service: Arc<PostgresService>,
    pub materials_service: Arc<MaterialsService>, // ДОБАВИТЬ ЭТО ПОЛЕ
}

impl AppState {
    pub async fn new(settings: Arc<AppSettings>, postgres_service: Arc<PostgresService>) -> Self {
        // Инициализируем сервис материалов
        let materials_repository = Arc::new(MaterialsRepository::new(
            postgres_service.connection.pool().clone().into()
        ));
        let materials_service = Arc::new(MaterialsService::new(materials_repository));
        
        Self {
            settings,
            postgres_service,
            materials_service,
        }
    }
}
