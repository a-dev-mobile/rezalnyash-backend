use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MaterialNameDb {
    pub material_name_uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

impl MaterialNameDb {
    pub fn new(material_name_uid: Uuid, name_ru: String, name_en: String) -> Self {
        Self {
            material_name_uid,
            name_ru,
            name_en,
        }
    }
}
