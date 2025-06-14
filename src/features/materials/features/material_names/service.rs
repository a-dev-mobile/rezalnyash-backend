use uuid::Uuid;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::features::materials::shared::errors::{MaterialError, MaterialResult};
use super::{entity::{MaterialName, MaterialNameUid}, repository::MaterialNameRepository};

/// DTO для создания названия материала
#[derive(Debug, Deserialize)]
pub struct CreateMaterialNameDto {
    pub name_ru: String,
    pub name_en: String,
}

/// DTO для ответа с названием материала
#[derive(Debug, Serialize)]
pub struct MaterialNameDto {
    pub uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

impl MaterialNameDto {
    pub fn from_entity(material_name: &MaterialName) -> Self {
        Self {
            uid: material_name.id().as_uuid(),
            name_ru: material_name.name_ru().to_string(),
            name_en: material_name.name_en().to_string(),
        }
    }
}

/// Трейт сервиса названий материалов
#[async_trait::async_trait]
pub trait MaterialNameService: Send + Sync {
    async fn get_material_name(&self, id: Uuid) -> MaterialResult<MaterialNameDto>;
    async fn get_all_material_names(&self) -> MaterialResult<Vec<MaterialNameDto>>;
    async fn create_material_name(&self, dto: CreateMaterialNameDto) -> MaterialResult<MaterialNameDto>;
    async fn material_name_exists(&self, id: Uuid) -> MaterialResult<bool>;
}

/// Реализация сервиса названий материалов
pub struct MaterialNameServiceImpl {
    repository: Arc<dyn MaterialNameRepository>,
}

impl MaterialNameServiceImpl {
    pub fn new(repository: Arc<dyn MaterialNameRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl MaterialNameService for MaterialNameServiceImpl {
    async fn get_material_name(&self, id: Uuid) -> Result<MaterialNameDto, MaterialError> {
        let material_name = self.repository.get_by_id(MaterialNameUid::from_uuid(id)).await?;
        Ok(MaterialNameDto::from_entity(&material_name))
    }

    async fn get_all_material_names(&self) -> Result<Vec<MaterialNameDto>, MaterialError> {
        let material_names = self.repository.get_all().await?;
        Ok(material_names.iter().map(MaterialNameDto::from_entity).collect())
    }

    async fn create_material_name(&self, dto: CreateMaterialNameDto) -> Result<MaterialNameDto, MaterialError> {
        let material_name = MaterialName::new(dto.name_ru, dto.name_en)?;
        let created_material_name = self.repository.create(&material_name).await?;
        Ok(MaterialNameDto::from_entity(&created_material_name))
    }

    async fn material_name_exists(&self, id: Uuid) -> Result<bool, MaterialError> {
        self.repository.exists(MaterialNameUid::from_uuid(id)).await
    }
}