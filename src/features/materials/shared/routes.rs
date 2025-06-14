use axum::Router;
use sqlx::PgPool;

use crate::features::materials::features::widths::routes::WidthRoutesBuilder;

/// Главный строитель всех материальных маршрутов
pub struct MaterialsRouter;

impl MaterialsRouter {
    /// Создает все маршруты материалов
    pub fn build_all(pool: PgPool) -> Router {
        Router::new()
            // .nest("/types", crate::features::materials::material_types::routes::MaterialTypeRoutesBuilder::build(pool.clone()))
            // .nest("/names", crate::features::materials::material_names::routes::MaterialNameRoutesBuilder::build(pool.clone()))
            .nest("/widths", WidthRoutesBuilder::build(pool.clone()))
            // .nest("/heights", crate::features::materials::heights::routes::HeightRoutesBuilder::build(pool.clone()))
            // .nest("/thicknesses", crate::features::materials::thicknesses::routes::ThicknessRoutesBuilder::build(pool))
    }



}