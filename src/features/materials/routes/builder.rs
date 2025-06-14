use crate::features::materials::{
    handlers::{
        traits::{HeightHandler, MaterialNameHandler, MaterialTypeHandler, ThicknessHandler, WidthHandler},
        v1::{HeightHandlerV1, MaterialNameHandlerV1, MaterialTypeHandlerV1, ThicknessHandlerV1, WidthHandlerV1},
    },
    repositories::{
        implementations::{
            PostgresHeightRepository, PostgresMaterialNameRepository, PostgresMaterialTypeRepository,
            PostgresThicknessRepository, PostgresWidthRepository,
        },
        traits::{
            HeightRepository, MaterialNameRepository, MaterialTypeRepository, ThicknessRepository, WidthRepository,
        },
    },
    routes::v1::materials_routes_v1,
    services::{
        implementations::{
            HeightServiceImpl, MaterialNameServiceImpl, MaterialTypeServiceImpl, ThicknessServiceImpl, WidthServiceImpl,
        },
        traits::{HeightService, MaterialNameService, MaterialTypeService, ThicknessService, WidthService},
    },
};

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

pub struct MaterialsRoutesBuilder;

impl MaterialsRoutesBuilder {
    /// Создает полный набор маршрутов для материалов v1 включая размеры
    pub fn build_v1(pool: PgPool) -> Router {
        // Создаем репозитории
        let material_type_repo: Arc<dyn MaterialTypeRepository> =
            Arc::new(PostgresMaterialTypeRepository::new(pool.clone()));
        let material_name_repo: Arc<dyn MaterialNameRepository> =
            Arc::new(PostgresMaterialNameRepository::new(pool.clone()));
        let width_repo: Arc<dyn WidthRepository> = Arc::new(PostgresWidthRepository::new(pool.clone()));
        let height_repo: Arc<dyn HeightRepository> = Arc::new(PostgresHeightRepository::new(pool.clone()));
        let thickness_repo: Arc<dyn ThicknessRepository> = Arc::new(PostgresThicknessRepository::new(pool));

        // Создаем сервисы
        let material_type_service: Arc<dyn MaterialTypeService> =
            Arc::new(MaterialTypeServiceImpl::new(material_type_repo));
        let material_name_service: Arc<dyn MaterialNameService> =
            Arc::new(MaterialNameServiceImpl::new(material_name_repo));
        let width_service: Arc<dyn WidthService> = Arc::new(WidthServiceImpl::new(width_repo));
        let height_service: Arc<dyn HeightService> = Arc::new(HeightServiceImpl::new(height_repo));
        let thickness_service: Arc<dyn ThicknessService> = Arc::new(ThicknessServiceImpl::new(thickness_repo));

        // Создаем handlers
        let material_type_handler: Arc<dyn MaterialTypeHandler> =
            Arc::new(MaterialTypeHandlerV1::new(material_type_service));
        let material_name_handler: Arc<dyn MaterialNameHandler> =
            Arc::new(MaterialNameHandlerV1::new(material_name_service));
        let width_handler: Arc<dyn WidthHandler> = Arc::new(WidthHandlerV1::new(width_service));
        let height_handler: Arc<dyn HeightHandler> = Arc::new(HeightHandlerV1::new(height_service));
        let thickness_handler: Arc<dyn ThicknessHandler> = Arc::new(ThicknessHandlerV1::new(thickness_service));

        // Возвращаем маршруты
        materials_routes_v1(
            material_type_handler,
            material_name_handler,
            width_handler,
            height_handler,
            thickness_handler,
        )
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

    /// Создает маршруты только для ширин
    pub fn build_widths_v1(pool: PgPool) -> Router {
        let width_repo: Arc<dyn WidthRepository> = Arc::new(PostgresWidthRepository::new(pool));
        let width_service: Arc<dyn WidthService> = Arc::new(WidthServiceImpl::new(width_repo));
        let width_handler: Arc<dyn WidthHandler> = Arc::new(WidthHandlerV1::new(width_service));

        crate::features::materials::routes::v1::width_routes(width_handler)
    }

    /// Создает маршруты только для высот
    pub fn build_heights_v1(pool: PgPool) -> Router {
        let height_repo: Arc<dyn HeightRepository> = Arc::new(PostgresHeightRepository::new(pool));
        let height_service: Arc<dyn HeightService> = Arc::new(HeightServiceImpl::new(height_repo));
        let height_handler: Arc<dyn HeightHandler> = Arc::new(HeightHandlerV1::new(height_service));

        crate::features::materials::routes::v1::height_routes(height_handler)
    }

    /// Создает маршруты только для толщин
    pub fn build_thicknesses_v1(pool: PgPool) -> Router {
        let thickness_repo: Arc<dyn ThicknessRepository> = Arc::new(PostgresThicknessRepository::new(pool));
        let thickness_service: Arc<dyn ThicknessService> = Arc::new(ThicknessServiceImpl::new(thickness_repo));
        let thickness_handler: Arc<dyn ThicknessHandler> = Arc::new(ThicknessHandlerV1::new(thickness_service));

        crate::features::materials::routes::v1::thickness_routes(thickness_handler)
    }
}
