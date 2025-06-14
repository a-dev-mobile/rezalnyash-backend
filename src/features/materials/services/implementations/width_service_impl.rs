use std::sync::Arc;
use uuid::Uuid;
use crate::features::materials::{
    domain::{entities::Width, errors::MaterialError, value_objects::WidthUid},
    repositories::traits::WidthRepository,
    services::{dto::{CreateWidthDto, WidthDto}, traits::WidthService},
};

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
        let width_id = WidthUid::new(id);
        let width = self.repository.get_width(&width_id).await?;
        Ok(WidthDto::from_domain(&width))
    }

    async fn get_all_widths(&self) -> Result<Vec<WidthDto>, MaterialError> {
        let widths = self.repository.get_all_widths().await?;
        Ok(widths.iter().map(WidthDto::from_domain).collect())
    }

    async fn create_width(&self, dto: CreateWidthDto) -> Result<WidthDto, MaterialError> {
        let width = Width::create(dto.width)?;
        let created_width = self.repository.create_width(width).await?;
        Ok(WidthDto::from_domain(&created_width))
    }

    async fn exists(&self, id: Uuid) -> Result<bool, MaterialError> {
        let width_id = WidthUid::new(id);
        self.repository.exists(&width_id).await
    }
}