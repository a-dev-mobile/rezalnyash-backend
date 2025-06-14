use uuid::Uuid;
use crate::features::materials::{
    domain::errors::MaterialError,
    services::dto::{CreateThicknessDto, ThicknessDto},
};

#[async_trait::async_trait]
pub trait ThicknessService: Send + Sync {
    async fn get_thickness(&self, id: Uuid) -> Result<ThicknessDto, MaterialError>;
    async fn get_all_thicknesses(&self) -> Result<Vec<ThicknessDto>, MaterialError>;
    async fn create_thickness(&self, dto: CreateThicknessDto) -> Result<ThicknessDto, MaterialError>;
    async fn exists(&self, id: Uuid) -> Result<bool, MaterialError>;
}