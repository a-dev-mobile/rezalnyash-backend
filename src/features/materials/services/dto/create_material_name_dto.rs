use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMaterialNameDto {
    pub name_ru: String,
    pub name_en: String,
}

impl CreateMaterialNameDto {
    pub fn new(name_ru: String, name_en: String) -> Self {
        Self { name_ru, name_en }
    }
}
