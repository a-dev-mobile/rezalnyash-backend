pub mod domain;
pub mod repositories;
pub mod services;
pub mod models;
pub mod handlers;
pub mod routes;

pub use domain::{
    entities::{MaterialType, MaterialName},
    value_objects::{MaterialTypeId, MaterialNameId},
    errors::MaterialError,
};

// Экспортируем основные трейты
pub use domain::traits::{MaterialTypeBehavior, MaterialNameBehavior};
pub use repositories::traits::{MaterialTypeRepository, MaterialNameRepository};
pub use services::traits::{MaterialTypeService, MaterialNameService};
pub use handlers::traits::{MaterialTypeHandler, MaterialNameHandler};

// Экспортируем builder для создания маршрутов
pub use routes::builder::MaterialsRoutesBuilder;