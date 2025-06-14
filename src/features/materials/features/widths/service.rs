use uuid::Uuid;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

use crate::features::materials::shared::errors::{MaterialError, MaterialResult};

use super::{entity::{Width, WidthUid}, repository::WidthRepository};

/// DTO для создания ширины
#[derive(Debug, Deserialize)]
pub struct CreateWidthDto {
    pub width: f64,
}

/// DTO для ответа с шириной
#[derive(Debug, Serialize)]
pub struct WidthDto {
    pub uid: Uuid,
    pub width: f64,
}

impl WidthDto {
    pub fn from_entity(width: &Width) -> Self {
        Self {
            uid: width.id().as_uuid(),
            width: width.value(),
        }
    }
}

/// Трейт сервиса ширин
#[async_trait::async_trait]
pub trait WidthService: Send + Sync {
    async fn get_width(&self, id: Uuid) -> MaterialResult<WidthDto>;
    async fn get_all_widths(&self) -> MaterialResult<Vec<WidthDto>>;
    async fn create_width(&self, dto: CreateWidthDto) -> MaterialResult<WidthDto>;
    async fn width_exists(&self, id: Uuid) -> MaterialResult<bool>;
}

/// Реализация сервиса ширин
pub struct WidthServiceImpl {
    repository: Arc<dyn WidthRepository>,
}

impl WidthServiceImpl {
    pub fn new(repository: Arc<dyn WidthRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl WidthService for WidthServiceImpl {
    async fn get_width(&self, id: Uuid) -> Result<WidthDto, MaterialError> {
        let width = self.repository.get_by_id(WidthUid::from_uuid(id)).await?;
        Ok(WidthDto::from_entity(&width))
    }

    async fn get_all_widths(&self) -> Result<Vec<WidthDto>, MaterialError> {
        let widths = self.repository.get_all().await?;
        Ok(widths.iter().map(WidthDto::from_entity).collect())
    }

    async fn create_width(&self, dto: CreateWidthDto) -> Result<WidthDto, MaterialError> {
        let width = Width::new(dto.width)?;
        let created_width = self.repository.create(&width).await?;
        Ok(WidthDto::from_entity(&created_width))
    }

    async fn width_exists(&self, id: Uuid) -> Result<bool, MaterialError> {
        self.repository.exists(WidthUid::from_uuid(id)).await
    }
}