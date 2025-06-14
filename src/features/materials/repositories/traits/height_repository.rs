use crate::features::materials::domain::{
    entities::Height, errors::MaterialError, value_objects::HeightUid, traits::HeightBehavior,
};

#[async_trait::async_trait]
pub trait HeightRepository: HeightBehavior + Send + Sync {
    async fn health_check(&self) -> Result<(), MaterialError>;
}
