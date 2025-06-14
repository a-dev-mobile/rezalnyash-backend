
// // widths/routes.rs - Маршруты
// use axum::{routing::{get, post}, Router};
// use sqlx::PgPool;
// use std::sync::Arc;
// use super::{
//     handler::{WidthHandler, WidthHandlerV1},
//     service::{WidthService, WidthServiceImpl},
//     repository::{WidthRepository, PostgresWidthRepository},
// };

// /// Создает маршруты для фичи ширин
// pub fn width_routes(handler: Arc<dyn WidthHandler>) -> Router {
//     Router::new()
//         .route("/", get({
//             let handler = Arc::clone(&handler);
//             move || async move { handler.get_all_widths().await }
//         }))
//         .route("/", post({
//             let handler = Arc::clone(&handler);
//             move |payload| async move { handler.create_width(payload).await }
//         }))
//         .route("/{id}", get({
//             let handler = Arc::clone(&handler);
//             move |path| async move { handler.get_width(path).await }
//         }))
// }

// /// Строитель маршрутов фичи
// pub struct WidthRoutesBuilder;

// impl WidthRoutesBuilder {
//     /// Создает полные маршруты с зависимостями
//     pub fn build(pool: PgPool) -> Router {
//         // Создаем зависимости
//         let repository: Arc<dyn WidthRepository> = Arc::new(PostgresWidthRepository::new(pool));
//         let service: Arc<dyn WidthService> = Arc::new(WidthServiceImpl::new(repository));
//         let handler: Arc<dyn WidthHandler> = Arc::new(WidthHandlerV1::new(service));

//         // Возвращаем маршруты
//         width_routes(handler)
//     }
// }