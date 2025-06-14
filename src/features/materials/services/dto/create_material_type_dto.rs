use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMaterialTypeDto {
    pub name_ru: String,
    pub name_en: String,
}

impl CreateMaterialTypeDto {
    pub fn new(name_ru: String, name_en: String) -> Self {
        Self { name_ru, name_en }
    }
}