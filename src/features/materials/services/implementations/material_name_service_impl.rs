
use std::sync::Arc;
use crate::features::materials::{
    domain::{
        entities::MaterialName,
        errors::MaterialError,
        value_objects::MaterialNameId,
    },
    repositories::traits::MaterialNameRepository,
    services::{
        dto::{CreateMaterialNameDto, MaterialNameDto},
        traits::MaterialNameService,
    },
};

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
    async fn get_material_name(&self, id: i32) -> Result<MaterialNameDto, MaterialError> {
        let material_name_id = MaterialNameId::new(id)?;
        let material_name = self.repository.get_material_name(&material_name_id).await?;
        Ok(MaterialNameDto::from_domain(&material_name))
    }

    async fn get_all_material_names(&self) -> Result<Vec<MaterialNameDto>, MaterialError> {
        let material_names = self.repository.get_all_material_names().await?;
        Ok(material_names
            .iter()
            .map(MaterialNameDto::from_domain)
            .collect())
    }

    async fn create_material_name(&self, dto: CreateMaterialNameDto) -> Result<MaterialNameDto, MaterialError> {
        // Валидация данных
        let material_name = MaterialName::create(dto.name_ru, dto.name_en)?;
        
        // Сохранение через репозиторий
        let created_material_name = self.repository.create_material_name(material_name).await?;
        
        Ok(MaterialNameDto::from_domain(&created_material_name))
    }

    async fn exists(&self, id: i32) -> Result<bool, MaterialError> {
        let material_name_id = MaterialNameId::new(id)?;
        self.repository.exists(&material_name_id).await
    }
}