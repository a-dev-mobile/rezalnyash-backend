use uuid::Uuid;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::features::materials::shared::errors::{MaterialError, MaterialResult};
use super::{entity::{MaterialType, MaterialTypeUid}, repository::TypeRepository};

/// DTO для создания типа материала
#[derive(Debug, Deserialize)]
pub struct CreateMaterialTypeDto {
    pub name_ru: String,
    pub name_en: String,
}

/// DTO для ответа с типом материала
#[derive(Debug, Serialize)]
pub struct MaterialTypeDto {
    pub uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

impl MaterialTypeDto {
    pub fn from_entity(material_type: &MaterialType) -> Self {
        Self {
            uid: material_type.id().as_uuid(),
            name_ru: material_type.name_ru().to_string(),
            name_en: material_type.name_en().to_string(),
        }
    }
}

/// Трейт сервиса типов материалов
#[async_trait::async_trait]
pub trait TypeService: Send + Sync {
    async fn get_type(&self, id: Uuid) -> MaterialResult<MaterialTypeDto>;
    async fn get_all_types(&self) -> MaterialResult<Vec<MaterialTypeDto>>;
    async fn create_type(&self, dto: CreateMaterialTypeDto) -> MaterialResult<MaterialTypeDto>;
    async fn material_type_exists(&self, id: Uuid) -> MaterialResult<bool>;
}

/// Реализация сервиса типов материалов
pub struct MaterialTypeServiceImpl {
    repository: Arc<dyn TypeRepository>,
}

impl MaterialTypeServiceImpl {
    pub fn new(repository: Arc<dyn TypeRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl TypeService for MaterialTypeServiceImpl {
    async fn get_type(&self, id: Uuid) -> Result<MaterialTypeDto, MaterialError> {
        let material_type = self.repository.get_by_id(MaterialTypeUid::from_uuid(id)).await?;
        Ok(MaterialTypeDto::from_entity(&material_type))
    }

    async fn get_all_types(&self) -> Result<Vec<MaterialTypeDto>, MaterialError> {
        let types = self.repository.get_all().await?;
        Ok(types.iter().map(MaterialTypeDto::from_entity).collect())
    }

    async fn create_type(&self, dto: CreateMaterialTypeDto) -> Result<MaterialTypeDto, MaterialError> {
        let material_type = MaterialType::new(dto.name_ru, dto.name_en)?;
        let created_material_type = self.repository.create(&material_type).await?;
        Ok(MaterialTypeDto::from_entity(&created_material_type))
    }

    async fn material_type_exists(&self, id: Uuid) -> Result<bool, MaterialError> {
        self.repository.exists(MaterialTypeUid::from_uuid(id)).await
    }
}