use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Уникальный идентификатор материала
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialUid(Uuid);

impl MaterialUid {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

/// Стандартный размер материала
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandardSize {
    pub width: f64,
    pub height: f64,
}

impl StandardSize {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

/// Доменная сущность материала
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    id: MaterialUid,
    type_name_ru: String,
    type_name_en: String,
    name_ru: String,
    name_en: String,
    standard_sizes: Vec<StandardSize>,
    default_thicknesses: Vec<f64>,
}

impl Material {
    /// Создать новый материал
    pub fn new(
        type_name_ru: String,
        type_name_en: String,
        name_ru: String,
        name_en: String,
        standard_sizes: Vec<StandardSize>,
        default_thicknesses: Vec<f64>,
    ) -> Self {
        Self {
            id: MaterialUid::new(),
            type_name_ru,
            type_name_en,
            name_ru,
            name_en,
            standard_sizes,
            default_thicknesses,
        }
    }

    /// Создать материал с существующим ID (из БД)
    pub fn from_db(
        id: MaterialUid,
        type_name_ru: String,
        type_name_en: String,
        name_ru: String,
        name_en: String,
        standard_sizes: Vec<StandardSize>,
        default_thicknesses: Vec<f64>,
    ) -> Self {
        Self {
            id,
            type_name_ru,
            type_name_en,
            name_ru,
            name_en,
            standard_sizes,
            default_thicknesses,
        }
    }

    // Геттеры
    pub fn id(&self) -> MaterialUid {
        self.id
    }

    pub fn type_name_ru(&self) -> &str {
        &self.type_name_ru
    }

    pub fn type_name_en(&self) -> &str {
        &self.type_name_en
    }

    pub fn name_ru(&self) -> &str {
        &self.name_ru
    }

    pub fn name_en(&self) -> &str {
        &self.name_en
    }

    pub fn standard_sizes(&self) -> &[StandardSize] {
        &self.standard_sizes
    }

    pub fn default_thicknesses(&self) -> &[f64] {
        &self.default_thicknesses
    }
}
