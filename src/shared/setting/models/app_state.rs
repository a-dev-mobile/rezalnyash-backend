use std::sync::Arc;

use crate::{
    features::{
        // Material types feature
        health::{
            handler::{HealthHandler, HealthHandlerV1},
            service::{HealthService, HealthServiceImpl},
        },

        materials::features::{
            heights::{
                handler::{HeightHandler, HeightHandlerV1},
                repository::{HeightRepository, PostgresHeightRepository},
                service::{HeightService, HeightServiceImpl},
            },
            names::{
                handler::{NameHandler, HandlerV1},
                repository::{NameRepository, PostgresNameRepository},
                service::{NameService, NameServiceImpl},
            },
            types::{
                handler::{TypeHandler, MaterialTypeHandlerV1},
                repository::{TypeRepository, PostgresMaterialTypeRepository},
                service::{TypeService, MaterialTypeServiceImpl},
            },
            thicknesses::{
                handler::{ThicknessHandler, ThicknessHandlerV1},
                repository::{PostgresThicknessRepository, ThicknessRepository},
                service::{ThicknessService, ThicknessServiceImpl},
            },
            widths::{
                handler::{WidthHandler, WidthHandlerV1},
                repository::{PostgresWidthRepository, WidthRepository},
                service::{WidthService, WidthServiceImpl},
            },
        },
    },
    shared::{database::service::PostgresService, setting::models::app_setting::AppSettings},
};

pub struct AppState {
    pub settings: Arc<AppSettings>,
    pub postgres_service: Arc<PostgresService>,

    // Material types feature dependencies
    pub material_type_handler: Arc<dyn TypeHandler>,
    pub material_type_service: Arc<dyn TypeService>,
    pub material_type_repository: Arc<dyn TypeRepository>,

    // Material names feature dependencies
    pub material_name_handler: Arc<dyn NameHandler>,
    pub material_name_service: Arc<dyn NameService>,
    pub material_name_repository: Arc<dyn NameRepository>,

    // Width feature dependencies
    pub material_width_handler: Arc<dyn WidthHandler>,
    pub material_width_service: Arc<dyn WidthService>,
    pub material_width_repository: Arc<dyn WidthRepository>,

    // Height feature dependencies
    pub material_height_handler: Arc<dyn HeightHandler>,
    pub material_height_service: Arc<dyn HeightService>,
    pub material_height_repository: Arc<dyn HeightRepository>,

    // Thickness feature dependencies
    pub material_thickness_handler: Arc<dyn ThicknessHandler>,
    pub material_thickness_service: Arc<dyn ThicknessService>,
    pub material_thickness_repository: Arc<dyn ThicknessRepository>,

    // Health feature dependencies
    pub material_health_handler: Arc<dyn HealthHandler>,
    pub material_health_service: Arc<dyn HealthService>,
}

impl AppState {
    pub async fn new(settings: Arc<AppSettings>, postgres_service: Arc<PostgresService>) -> Self {
        // Получаем pool из postgres_service
        let pool = postgres_service.connection.pool().clone();

        // Создаем зависимости для material types feature
        let material_type_repository: Arc<dyn TypeRepository> =
            Arc::new(PostgresMaterialTypeRepository::new(pool.clone()));
        let material_type_service: Arc<dyn TypeService> =
            Arc::new(MaterialTypeServiceImpl::new(material_type_repository.clone()));
        let material_type_handler: Arc<dyn TypeHandler> =
            Arc::new(MaterialTypeHandlerV1::new(material_type_service.clone()));

        // Создаем зависимости для material names feature
        let material_name_repository: Arc<dyn NameRepository> =
            Arc::new(PostgresNameRepository::new(pool.clone()));
        let material_name_service: Arc<dyn NameService> =
            Arc::new(NameServiceImpl::new(material_name_repository.clone()));
        let material_name_handler: Arc<dyn NameHandler> =
            Arc::new(HandlerV1::new(material_name_service.clone()));

        // Создаем зависимости для widths feature
        let width_repository: Arc<dyn WidthRepository> = Arc::new(PostgresWidthRepository::new(pool.clone()));
        let width_service: Arc<dyn WidthService> = Arc::new(WidthServiceImpl::new(width_repository.clone()));
        let width_handler: Arc<dyn WidthHandler> = Arc::new(WidthHandlerV1::new(width_service.clone()));

        // Создаем зависимости для heights feature
        let height_repository: Arc<dyn HeightRepository> = Arc::new(PostgresHeightRepository::new(pool.clone()));
        let height_service: Arc<dyn HeightService> = Arc::new(HeightServiceImpl::new(height_repository.clone()));
        let height_handler: Arc<dyn HeightHandler> = Arc::new(HeightHandlerV1::new(height_service.clone()));

        // Создаем зависимости для thickness feature
        let thickness_repository: Arc<dyn ThicknessRepository> =
            Arc::new(PostgresThicknessRepository::new(pool.clone()));
        let thickness_service: Arc<dyn ThicknessService> =
            Arc::new(ThicknessServiceImpl::new(thickness_repository.clone()));
        let thickness_handler: Arc<dyn ThicknessHandler> = Arc::new(ThicknessHandlerV1::new(thickness_service.clone()));

        // Создаем зависимости для health feature
        let health_service: Arc<dyn HealthService> = Arc::new(HealthServiceImpl::new(settings.clone(), pool.clone()));
        let health_handler: Arc<dyn HealthHandler> = Arc::new(HealthHandlerV1::new(health_service.clone()));

        Self {
            settings,
            postgres_service,
            material_type_handler,
            material_type_service,
            material_type_repository,
            material_name_handler,
            material_name_service,
            material_name_repository,
            material_width_handler: width_handler,
            material_width_service: width_service,
            material_width_repository: width_repository,
            material_height_handler: height_handler,
            material_height_service: height_service,
            material_height_repository: height_repository,
            material_thickness_handler: thickness_handler,
            material_thickness_service: thickness_service,
            material_thickness_repository: thickness_repository,
            material_health_handler: health_handler,
            material_health_service: health_service,
        }
    }
}
