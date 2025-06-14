use std::sync::Arc;
use uuid::Uuid;
use crate::features::materials::{
    domain::{entities::Thickness, errors::MaterialError, value_objects::ThicknessUid},
    repositories::traits::ThicknessRepository,
    services::{dto::{CreateThicknessDto, ThicknessDto}, traits::ThicknessService},
};

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
        let thickness_id = ThicknessUid::new(id);
        let thickness = self.repository.get_thickness(&thickness_id).await?;
        Ok(ThicknessDto::from_domain(&thickness))
    }

    async fn get_all_thicknesses(&self) -> Result<Vec<ThicknessDto>, MaterialError> {
        let thicknesses = self.repository.get_all_thicknesses().await?;
        Ok(thicknesses.iter().map(ThicknessDto::from_domain).collect())
    }

    async fn create_thickness(&self, dto: CreateThicknessDto) -> Result<ThicknessDto, MaterialError> {
        let thickness = Thickness::create(dto.thickness)?;
        let created_thickness = self.repository.create_thickness(thickness).await?;
        Ok(ThicknessDto::from_domain(&created_thickness))
    }

    async fn exists(&self, id: Uuid) -> Result<bool, MaterialError> {
        let thickness_id = ThicknessUid::new(id);
        self.repository.exists(&thickness_id).await
    }
}