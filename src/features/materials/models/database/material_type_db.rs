use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MaterialTypeDb {
    pub material_type_id: i32,
    pub name_ru: String,
    pub name_en: String,
}

impl MaterialTypeDb {
    pub fn new(material_type_id: i32, name_ru: String, name_en: String) -> Self {
        Self {
            material_type_id,
            name_ru,
            name_en,
        }
    }
}
