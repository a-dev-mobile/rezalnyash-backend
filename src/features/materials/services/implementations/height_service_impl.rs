use std::sync::Arc;
use uuid::Uuid;
use crate::features::materials::{
    domain::{entities::Height, errors::MaterialError, value_objects::HeightUid},
    repositories::traits::HeightRepository,
    services::{dto::{CreateHeightDto, HeightDto}, traits::HeightService},
};

pub struct HeightServiceImpl {
    repository: Arc<dyn HeightRepository>,
}

impl HeightServiceImpl {
    pub fn new(repository: Arc<dyn HeightRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl HeightService for HeightServiceImpl {
    async fn get_height(&self, id: Uuid) -> Result<HeightDto, MaterialError> {
        let height_id = HeightUid::new(id);
        let height = self.repository.get_height(&height_id).await?;
        Ok(HeightDto::from_domain(&height))
    }

    async fn get_all_heights(&self) -> Result<Vec<HeightDto>, MaterialError> {
        let heights = self.repository.get_all_heights().await?;
        Ok(heights.iter().map(HeightDto::from_domain).collect())
    }

    async fn create_height(&self, dto: CreateHeightDto) -> Result<HeightDto, MaterialError> {
        let height = Height::create(dto.height)?;
        let created_height = self.repository.create_height(height).await?;
        Ok(HeightDto::from_domain(&created_height))
    }

    async fn exists(&self, id: Uuid) -> Result<bool, MaterialError> {
        let height_id = HeightUid::new(id);
        self.repository.exists(&height_id).await
    }
}
