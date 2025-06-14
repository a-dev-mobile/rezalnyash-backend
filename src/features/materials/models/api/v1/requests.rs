use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMaterialTypeRequest {
    pub name_ru: String,
    pub name_en: String,
}

impl CreateMaterialTypeRequest {
    pub fn new(name_ru: String, name_en: String) -> Self {
        Self { name_ru, name_en }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMaterialNameRequest {
    pub name_ru: String,
    pub name_en: String,
}

impl CreateMaterialNameRequest {
    pub fn new(name_ru: String, name_en: String) -> Self {
        Self { name_ru, name_en }
    }
}