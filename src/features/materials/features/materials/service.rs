// src/features/materials/features/materials/service.rs
use std::sync::Arc;
use serde::Serialize;
use crate::features::materials::shared::errors::{MaterialError, MaterialResult};
use super::{entity::{Material, StandardSize}, repository::MaterialRepository};

/// DTO для стандартного размера
#[derive(Debug, Serialize)]
pub struct StandardSizeDto {
    pub width: f64,
    pub height: f64,
}

impl From<&StandardSize> for StandardSizeDto {
    fn from(size: &StandardSize) -> Self {
        Self {
            width: size.width,
            height: size.height,
        }
    }
}

/// DTO для ответа с материалом
#[derive(Debug, Serialize)]
pub struct MaterialDto {
    #[serde(rename = "type")]
    pub material_type: String,
    pub name_ru: String,
    pub name_en: String,
    pub standard_sizes: Vec<StandardSizeDto>,
    pub default_thicknesses: Vec<f64>,
}

impl MaterialDto {
    pub fn from_entity(material: &Material) -> Self {
        Self {
            material_type: format!("{} / {}", material.type_name_ru(), material.type_name_en()),
            name_ru: material.name_ru().to_string(),
            name_en: material.name_en().to_string(),
            standard_sizes: material
                .standard_sizes()
                .iter()
                .map(StandardSizeDto::from)
                .collect(),
            default_thicknesses: material.default_thicknesses().to_vec(),
        }
    }
}

/// DTO для списка материалов
#[derive(Debug, Serialize)]
pub struct MaterialsListDto {
    pub materials: Vec<MaterialDto>,
}

/// Трейт сервиса материалов
#[async_trait::async_trait]
pub trait MaterialService: Send + Sync {
    async fn get_all_materials(&self) -> MaterialResult<MaterialsListDto>;
}

/// Реализация сервиса материалов
pub struct MaterialServiceImpl {
    repository: Arc<dyn MaterialRepository>,
}

impl MaterialServiceImpl {
    pub fn new(repository: Arc<dyn MaterialRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl MaterialService for MaterialServiceImpl {
    async fn get_all_materials(&self) -> Result<MaterialsListDto, MaterialError> {
        let materials = self.repository.get_all().await?;
        let material_dtos: Vec<MaterialDto> = materials
            .iter()
            .map(MaterialDto::from_entity)
            .collect();

        Ok(MaterialsListDto {
            materials: material_dtos,
        })
    }
}