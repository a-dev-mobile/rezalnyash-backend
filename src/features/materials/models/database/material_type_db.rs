use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MaterialTypeDb {
    pub material_type_uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

impl MaterialTypeDb {
    pub fn new(material_type_uid: Uuid, name_ru: String, name_en: String) -> Self {
        Self {
            material_type_uid,
            name_ru,
            name_en,
        }
    }
}
