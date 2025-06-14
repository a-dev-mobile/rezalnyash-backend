use uuid::Uuid;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use crate::features::materials::shared::errors::{MaterialError, MaterialResult};
use super::{entity::{Type, TypeUid}, repository::TypeRepository};

/// DTO для создания типа материала
#[derive(Debug, Deserialize)]
pub struct CreateTypeDto {
    pub name_ru: String,
    pub name_en: String,
}

/// DTO для ответа с типом материала
#[derive(Debug, Serialize)]
pub struct TypeDto {
    pub uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

impl TypeDto {
    pub fn from_entity(material_type: &Type) -> Self {
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
    async fn get_type(&self, id: Uuid) -> MaterialResult<TypeDto>;
    async fn get_all_types(&self) -> MaterialResult<Vec<TypeDto>>;
    async fn create_type(&self, dto: CreateTypeDto) -> MaterialResult<TypeDto>;
    async fn type_exists(&self, id: Uuid) -> MaterialResult<bool>;
}

/// Реализация сервиса типов материалов
pub struct TypeServiceImpl {
    repository: Arc<dyn TypeRepository>,
}

impl TypeServiceImpl {
    pub fn new(repository: Arc<dyn TypeRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl TypeService for TypeServiceImpl {
    async fn get_type(&self, id: Uuid) -> Result<TypeDto, MaterialError> {
        let material_type = self.repository.get_by_id(TypeUid::from_uuid(id)).await?;
        Ok(TypeDto::from_entity(&material_type))
    }

    async fn get_all_types(&self) -> Result<Vec<TypeDto>, MaterialError> {
        let types = self.repository.get_all().await?;
        Ok(types.iter().map(TypeDto::from_entity).collect())
    }

    async fn create_type(&self, dto: CreateTypeDto) -> Result<TypeDto, MaterialError> {
        let material_type = Type::new(dto.name_ru, dto.name_en)?;
        let created_material_type = self.repository.create(&material_type).await?;
        Ok(TypeDto::from_entity(&created_material_type))
    }

    async fn type_exists(&self, id: Uuid) -> Result<bool, MaterialError> {
        self.repository.exists(TypeUid::from_uuid(id)).await
    }
}