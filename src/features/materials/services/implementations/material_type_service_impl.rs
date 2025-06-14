
use std::sync::Arc;
use crate::features::materials::{
    domain::{
        entities::MaterialType,
        errors::MaterialError,
        value_objects::MaterialTypeId,
    },
    repositories::traits::MaterialTypeRepository,
    services::{
        dto::{CreateMaterialTypeDto, MaterialTypeDto},
        traits::MaterialTypeService,
    },
};

pub struct MaterialTypeServiceImpl {
    repository: Arc<dyn MaterialTypeRepository>,
}

impl MaterialTypeServiceImpl {
    pub fn new(repository: Arc<dyn MaterialTypeRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl MaterialTypeService for MaterialTypeServiceImpl {
    async fn get_material_type(&self, id: i32) -> Result<MaterialTypeDto, MaterialError> {
        let material_type_id = MaterialTypeId::new(id)?;
        let material_type = self.repository.get_material_type(&material_type_id).await?;
        Ok(MaterialTypeDto::from_domain(&material_type))
    }

    async fn get_all_material_types(&self) -> Result<Vec<MaterialTypeDto>, MaterialError> {
        let material_types = self.repository.get_all_material_types().await?;
        Ok(material_types
            .iter()
            .map(MaterialTypeDto::from_domain)
            .collect())
    }

    async fn create_material_type(&self, dto: CreateMaterialTypeDto) -> Result<MaterialTypeDto, MaterialError> {
        // Валидация данных
        let material_type = MaterialType::create(dto.name_ru, dto.name_en)?;
        
        // Сохранение через репозиторий
        let created_material_type = self.repository.create_material_type(material_type).await?;
        
        Ok(MaterialTypeDto::from_domain(&created_material_type))
    }

    async fn exists(&self, id: i32) -> Result<bool, MaterialError> {
        let material_type_id = MaterialTypeId::new(id)?;
        self.repository.exists(&material_type_id).await
    }
}