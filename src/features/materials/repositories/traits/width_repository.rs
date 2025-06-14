use crate::features::materials::domain::{
    entities::Width, errors::MaterialError, value_objects::WidthUid, traits::WidthBehavior,
};

#[async_trait::async_trait]
pub trait WidthRepository: WidthBehavior + Send + Sync {
    async fn health_check(&self) -> Result<(), MaterialError>;
}
