use crate::features::health::handlers::traits::HealthHandler;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn health_routes_v1(handler: Arc<dyn HealthHandler>) -> Router {
    Router::new()
        .route(
            "/",
            get({
                let handler = Arc::clone(&handler);
                move || async move { handler.check_application_health().await }
            }),
        )
        .route(
            "/db",
            get({
                let handler = Arc::clone(&handler);
                move || async move { handler.check_database_health().await }
            }),
        )
}