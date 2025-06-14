use uuid::Uuid;
use crate::features::materials::{
    domain::errors::MaterialError,
    services::dto::{CreateWidthDto, WidthDto},
};

#[async_trait::async_trait]
pub trait WidthService: Send + Sync {
    async fn get_width(&self, id: Uuid) -> Result<WidthDto, MaterialError>;
    async fn get_all_widths(&self) -> Result<Vec<WidthDto>, MaterialError>;
    async fn create_width(&self, dto: CreateWidthDto) -> Result<WidthDto, MaterialError>;
    async fn exists(&self, id: Uuid) -> Result<bool, MaterialError>;
}