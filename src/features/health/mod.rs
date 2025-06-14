pub mod domain;
pub mod repositories;
pub mod services;
pub mod models;
pub mod handlers;
pub mod routes;

pub use domain::errors::HealthError;

// Экспортируем основные трейты
pub use domain::traits::HealthBehavior;
pub use repositories::traits::HealthRepository;
pub use services::traits::HealthService;
pub use handlers::traits::HealthHandler;

// Экспортируем builder для создания маршрутов
pub use routes::builder::HealthRoutesBuilder;