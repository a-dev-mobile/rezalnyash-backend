use crate::features::materials::domain::{entities::MaterialType, errors::MaterialError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialTypeDto {
    pub uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

impl MaterialTypeDto {
    pub fn new(uid: Uuid, name_ru: String, name_en: String) -> Self {
        Self { uid, name_ru, name_en }
    }

    /// Конвертация из доменной модели
    pub fn from_domain(domain: &MaterialType) -> Self {
        Self {
            uid: domain.id().value(),
            name_ru: domain.name_ru().to_string(),
            name_en: domain.name_en().to_string(),
        }
    }

    /// Конвертация в доменную модель
    pub fn to_domain(&self) -> Result<MaterialType, MaterialError> {
        use crate::features::materials::domain::value_objects::MaterialTypeUid;

        let uid = MaterialTypeUid::new(self.uid);
        MaterialType::new(uid, self.name_ru.clone(), self.name_en.clone())
    }
}
