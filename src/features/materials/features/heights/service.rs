use uuid::Uuid;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

use crate::features::materials::shared::errors::{MaterialError, MaterialResult};

use super::{entity::{Height, HeightUid}, repository::HeightRepository};

/// DTO для создания высоты
#[derive(Debug, Deserialize)]
pub struct CreateHeightDto {
    pub height: f64,
}

/// DTO для ответа с высотой
#[derive(Debug, Serialize)]
pub struct HeightDto {
    pub uid: Uuid,
    pub height: f64,
}

impl HeightDto {
    pub fn from_entity(height: &Height) -> Self {
        Self {
            uid: height.id().as_uuid(),
            height: height.value(),
        }
    }
}

/// Трейт сервиса высот
#[async_trait::async_trait]
pub trait HeightService: Send + Sync {
    async fn get_height(&self, id: Uuid) -> MaterialResult<HeightDto>;
    async fn get_all_heights(&self) -> MaterialResult<Vec<HeightDto>>;
    async fn create_height(&self, dto: CreateHeightDto) -> MaterialResult<HeightDto>;
    async fn height_exists(&self, id: Uuid) -> MaterialResult<bool>;
}

/// Реализация сервиса высот
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
        let height = self.repository.get_by_id(HeightUid::from_uuid(id)).await?;
        Ok(HeightDto::from_entity(&height))
    }

    async fn get_all_heights(&self) -> Result<Vec<HeightDto>, MaterialError> {
        let heights = self.repository.get_all().await?;
        Ok(heights.iter().map(HeightDto::from_entity).collect())
    }

    async fn create_height(&self, dto: CreateHeightDto) -> Result<HeightDto, MaterialError> {
        let height = Height::new(dto.height)?;
        let created_height = self.repository.create(&height).await?;
        Ok(HeightDto::from_entity(&created_height))
    }

    async fn height_exists(&self, id: Uuid) -> Result<bool, MaterialError> {
        self.repository.exists(HeightUid::from_uuid(id)).await
    }
}
