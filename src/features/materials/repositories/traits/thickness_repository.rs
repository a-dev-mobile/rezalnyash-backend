use crate::features::materials::domain::{
    entities::Thickness, errors::MaterialError, value_objects::ThicknessUid, traits::ThicknessBehavior,
};

#[async_trait::async_trait]
pub trait ThicknessRepository: ThicknessBehavior + Send + Sync {
    async fn health_check(&self) -> Result<(), MaterialError>;
}