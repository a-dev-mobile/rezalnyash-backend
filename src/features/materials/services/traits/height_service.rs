use uuid::Uuid;
use crate::features::materials::{
    domain::errors::MaterialError,
    services::dto::{CreateHeightDto, HeightDto},
};

#[async_trait::async_trait]
pub trait HeightService: Send + Sync {
    async fn get_height(&self, id: Uuid) -> Result<HeightDto, MaterialError>;
    async fn get_all_heights(&self) -> Result<Vec<HeightDto>, MaterialError>;
    async fn create_height(&self, dto: CreateHeightDto) -> Result<HeightDto, MaterialError>;
    async fn exists(&self, id: Uuid) -> Result<bool, MaterialError>;
}
