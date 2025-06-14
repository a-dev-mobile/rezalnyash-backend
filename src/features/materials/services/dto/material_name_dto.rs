
use serde::{Deserialize, Serialize};
use crate::features::materials::domain::{
    entities::MaterialName,
    errors::MaterialError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialNameDto {
    pub id: i32,
    pub name_ru: String,
    pub name_en: String,
}

impl MaterialNameDto {
    pub fn new(id: i32, name_ru: String, name_en: String) -> Self {
        Self { id, name_ru, name_en }
    }

    /// Конвертация из доменной модели
    pub fn from_domain(domain: &MaterialName) -> Self {
        Self {
            id: domain.id().value(),
            name_ru: domain.name_ru().to_string(),
            name_en: domain.name_en().to_string(),
        }
    }

    /// Конвертация в доменную модель
    pub fn to_domain(&self) -> Result<MaterialName, MaterialError> {
        use crate::features::materials::domain::value_objects::MaterialNameId;
        
        let id = MaterialNameId::new(self.id)?;
        MaterialName::new(id, self.name_ru.clone(), self.name_en.clone())
    }
}