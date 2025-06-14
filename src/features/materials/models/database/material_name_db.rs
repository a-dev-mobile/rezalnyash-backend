use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MaterialNameDb {
    pub material_name_id: i32,
    pub name_ru: String,
    pub name_en: String,
}

impl MaterialNameDb {
    pub fn new(material_name_id: i32, name_ru: String, name_en: String) -> Self {
        Self {
            material_name_id,
            name_ru,
            name_en,
        }
    }
}
