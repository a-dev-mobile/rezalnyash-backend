use uuid::Uuid;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::features::materials::shared::errors::{MaterialError, MaterialResult};
use super::{entity::{Name, NameUid}, repository::NameRepository};

/// DTO для создания названия материала
#[derive(Debug, Deserialize)]
pub struct CreateNameDto {
    pub name_ru: String,
    pub name_en: String,
}

/// DTO для ответа с названием материала
#[derive(Debug, Serialize)]
pub struct NameDto {
    pub uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

impl NameDto {
    pub fn from_entity(material_name: &Name) -> Self {
        Self {
            uid: material_name.id().as_uuid(),
            name_ru: material_name.name_ru().to_string(),
            name_en: material_name.name_en().to_string(),
        }
    }
}

/// Трейт сервиса названий материалов
#[async_trait::async_trait]
pub trait NameService: Send + Sync {
    async fn get_name(&self, id: Uuid) -> MaterialResult<NameDto>;
    async fn get_all_names(&self) -> MaterialResult<Vec<NameDto>>;
    async fn create_name(&self, dto: CreateNameDto) -> MaterialResult<NameDto>;
    async fn name_exists(&self, id: Uuid) -> MaterialResult<bool>;
}

/// Реализация сервиса названий материалов
pub struct NameServiceImpl {
    repository: Arc<dyn NameRepository>,
}

impl NameServiceImpl {
    pub fn new(repository: Arc<dyn NameRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl NameService for NameServiceImpl {
    async fn get_name(&self, id: Uuid) -> Result<NameDto, MaterialError> {
        let material_name = self.repository.get_by_id(NameUid::from_uuid(id)).await?;
        Ok(NameDto::from_entity(&material_name))
    }

    async fn get_all_names(&self) -> Result<Vec<NameDto>, MaterialError> {
        let names = self.repository.get_all().await?;
        Ok(names.iter().map(NameDto::from_entity).collect())
    }

    async fn create_name(&self, dto: CreateNameDto) -> Result<NameDto, MaterialError> {
        let material_name = Name::new(dto.name_ru, dto.name_en)?;
        let created_material_name = self.repository.create(&material_name).await?;
        Ok(NameDto::from_entity(&created_material_name))
    }

    async fn name_exists(&self, id: Uuid) -> Result<bool, MaterialError> {
        self.repository.exists(NameUid::from_uuid(id)).await
    }
}