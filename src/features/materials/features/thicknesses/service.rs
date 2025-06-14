use uuid::Uuid;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::features::materials::shared::errors::{MaterialError, MaterialResult};
use super::{entity::{Thickness, ThicknessUid}, repository::ThicknessRepository};

/// DTO для создания толщины
#[derive(Debug, Deserialize)]
pub struct CreateThicknessDto {
    pub thickness: f64,
}

/// DTO для ответа с толщиной
#[derive(Debug, Serialize)]
pub struct ThicknessDto {
    pub uid: Uuid,
    pub thickness: f64,
}

impl ThicknessDto {
    pub fn from_entity(thickness: &Thickness) -> Self {
        Self {
            uid: thickness.id().as_uuid(),
            thickness: thickness.value(),
        }
    }
}

/// Трейт сервиса толщин
#[async_trait::async_trait]
pub trait ThicknessService: Send + Sync {
    async fn get_thickness(&self, id: Uuid) -> MaterialResult<ThicknessDto>;
    async fn get_all_thicknesses(&self) -> MaterialResult<Vec<ThicknessDto>>;
    async fn create_thickness(&self, dto: CreateThicknessDto) -> MaterialResult<ThicknessDto>;
    async fn thickness_exists(&self, id: Uuid) -> MaterialResult<bool>;
}

/// Реализация сервиса толщин
pub struct ThicknessServiceImpl {
    repository: Arc<dyn ThicknessRepository>,
}

impl ThicknessServiceImpl {
    pub fn new(repository: Arc<dyn ThicknessRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl ThicknessService for ThicknessServiceImpl {
    async fn get_thickness(&self, id: Uuid) -> Result<ThicknessDto, MaterialError> {
        let thickness = self.repository.get_by_id(ThicknessUid::from_uuid(id)).await?;
        Ok(ThicknessDto::from_entity(&thickness))
    }

    async fn get_all_thicknesses(&self) -> Result<Vec<ThicknessDto>, MaterialError> {
        let thicknesses = self.repository.get_all().await?;
        Ok(thicknesses.iter().map(ThicknessDto::from_entity).collect())
    }

    async fn create_thickness(&self, dto: CreateThicknessDto) -> Result<ThicknessDto, MaterialError> {
        let thickness = Thickness::new(dto.thickness)?;
        let created_thickness = self.repository.create(&thickness).await?;
        Ok(ThicknessDto::from_entity(&created_thickness))
    }

    async fn thickness_exists(&self, id: Uuid) -> Result<bool, MaterialError> {
        self.repository.exists(ThicknessUid::from_uuid(id)).await
    }
}