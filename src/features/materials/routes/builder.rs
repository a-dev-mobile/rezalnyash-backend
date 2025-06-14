use crate::features::materials::{
    handlers::{
        traits::{MaterialNameHandler, MaterialTypeHandler},
        v1::{MaterialNameHandlerV1, MaterialTypeHandlerV1},
    },
    repositories::{
        implementations::{PostgresMaterialNameRepository, PostgresMaterialTypeRepository},
        traits::{MaterialNameRepository, MaterialTypeRepository},
    },
    routes::v1::materials_routes_v1,
    services::{
        implementations::{MaterialNameServiceImpl, MaterialTypeServiceImpl},
        traits::{MaterialNameService, MaterialTypeService},
    },
};
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

pub struct MaterialsRoutesBuilder;

impl MaterialsRoutesBuilder {
    /// Создает полный набор маршрутов для материалов v1
    pub fn build_v1(pool: PgPool) -> Router {
        // Создаем репозитории
        let material_type_repo: Arc<dyn MaterialTypeRepository> =
            Arc::new(PostgresMaterialTypeRepository::new(pool.clone()));
        let material_name_repo: Arc<dyn MaterialNameRepository> = Arc::new(PostgresMaterialNameRepository::new(pool));

        // Создаем сервисы
        let material_type_service: Arc<dyn MaterialTypeService> =
            Arc::new(MaterialTypeServiceImpl::new(material_type_repo));
        let material_name_service: Arc<dyn MaterialNameService> =
            Arc::new(MaterialNameServiceImpl::new(material_name_repo));

        // Создаем handlers
        let material_type_handler: Arc<dyn MaterialTypeHandler> =
            Arc::new(MaterialTypeHandlerV1::new(material_type_service));
        let material_name_handler: Arc<dyn MaterialNameHandler> =
            Arc::new(MaterialNameHandlerV1::new(material_name_service));

        // Возвращаем маршруты
        materials_routes_v1(material_type_handler, material_name_handler)
    }

    /// Создает маршруты только для типов материалов
    pub fn build_material_types_v1(pool: PgPool) -> Router {
        let material_type_repo: Arc<dyn MaterialTypeRepository> = Arc::new(PostgresMaterialTypeRepository::new(pool));
        let material_type_service: Arc<dyn MaterialTypeService> =
            Arc::new(MaterialTypeServiceImpl::new(material_type_repo));
        let material_type_handler: Arc<dyn MaterialTypeHandler> =
            Arc::new(MaterialTypeHandlerV1::new(material_type_service));

        crate::features::materials::routes::v1::material_type_routes(material_type_handler)
    }

    /// Создает маршруты только для названий материалов
    pub fn build_material_names_v1(pool: PgPool) -> Router {
        let material_name_repo: Arc<dyn MaterialNameRepository> = Arc::new(PostgresMaterialNameRepository::new(pool));
        let material_name_service: Arc<dyn MaterialNameService> =
            Arc::new(MaterialNameServiceImpl::new(material_name_repo));
        let material_name_handler: Arc<dyn MaterialNameHandler> =
            Arc::new(MaterialNameHandlerV1::new(material_name_service));

        crate::features::materials::routes::v1::material_name_routes(material_name_handler)
    }
}
