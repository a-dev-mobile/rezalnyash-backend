
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Модель материала из базы данных
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Material {
    pub id: Uuid,
    pub material_type: String,
    pub name_ru: String,
    pub name_en: Option<String>,
    pub thickness: Option<f64>,
    pub color: Option<String>,
    pub grain_direction: bool,
    pub created_at: DateTime<Utc>,
}

/// Стандартный размер листа материала
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MaterialStandardSize {

    pub material_id: Uuid,
    pub width: f64,
    pub height: f64,
    pub name: Option<String>,
    pub common_usage: Option<String>,

}

/// Пресет материала для API ответа
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialPreset {
    pub material_type: String,
    pub name_ru: String,
    pub name_en: Option<String>,
    pub standard_sizes: Vec<StandardSize>,
    pub default_thicknesses: Vec<f64>,
    pub properties: MaterialProperties,
}

/// Стандартный размер для API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardSize {
    pub width: f64,
    pub height: f64,
    pub name: Option<String>,
    pub common_usage: Option<String>,
}

/// Свойства материала
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialProperties {
    pub can_rotate: bool,
    pub has_grain: bool,
    pub recommended_blade_width: f64,
    pub recommended_edge_margin: f64,
}

/// Ответ API для списка материалов
#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialsResponse {
    pub materials: Vec<MaterialPreset>,
}
