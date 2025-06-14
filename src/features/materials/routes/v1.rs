use crate::features::materials::handlers::{
    traits::{MaterialNameHandler, MaterialTypeHandler},
    v1::{MaterialNameHandlerV1, MaterialTypeHandlerV1},
};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn material_type_routes(handler: Arc<dyn MaterialTypeHandler>) -> Router {
    Router::new()
        .route(
            "/",
            get({
                let handler = Arc::clone(&handler);
                move || async move { handler.get_all_material_types().await }
            }),
        )
        .route(
            "/",
            post({
                let handler = Arc::clone(&handler);
                move |payload| async move { handler.create_material_type(payload).await }
            }),
        )
        .route(
            "/{id}",
            get({
                let handler = Arc::clone(&handler);
                move |path| async move { handler.get_material_type(path).await }
            }),
        )
}

pub fn material_name_routes(handler: Arc<dyn MaterialNameHandler>) -> Router {
    Router::new()
        .route(
            "/",
            get({
                let handler = Arc::clone(&handler);
                move || async move { handler.get_all_material_names().await }
            }),
        )
        .route(
            "/",
            post({
                let handler = Arc::clone(&handler);
                move |payload| async move { handler.create_material_name(payload).await }
            }),
        )
        .route(
            "/{id}",
            get({
                let handler = Arc::clone(&handler);
                move |path| async move { handler.get_material_name(path).await }
            }),
        )
}

pub fn materials_routes_v1(
    material_type_handler: Arc<dyn MaterialTypeHandler>,
    material_name_handler: Arc<dyn MaterialNameHandler>,
) -> Router {
    Router::new()
        .nest("/types", material_type_routes(material_type_handler))
        .nest("/names", material_name_routes(material_name_handler))
}
