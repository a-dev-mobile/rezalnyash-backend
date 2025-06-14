pub mod domain;
pub mod repositories;
pub mod services;
pub mod models;
pub mod handlers;
pub mod routes;

pub use domain::{
    entities::{MaterialType, MaterialName, Width, Height, Thickness},
    value_objects::{MaterialTypeUid, MaterialNameUid, WidthUid, HeightUid, ThicknessUid},
    errors::MaterialError,
};

// Экспортируем основные трейты
pub use domain::traits::{
    MaterialTypeBehavior, MaterialNameBehavior, 
    WidthBehavior, HeightBehavior, ThicknessBehavior
};
pub use repositories::traits::{
    MaterialTypeRepository, MaterialNameRepository,
    WidthRepository, HeightRepository, ThicknessRepository
};
pub use services::traits::{
    MaterialTypeService, MaterialNameService,
    WidthService, HeightService, ThicknessService
};
pub use handlers::traits::{
    MaterialTypeHandler, MaterialNameHandler,
    WidthHandler, HeightHandler, ThicknessHandler
};

// Экспортируем builder для создания маршрутов
pub use routes::builder::MaterialsRoutesBuilder;