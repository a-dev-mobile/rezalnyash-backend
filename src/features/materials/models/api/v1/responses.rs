use crate::features::materials::services::dto::{MaterialNameDto, MaterialTypeDto};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialTypeResponse {
    pub uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

impl MaterialTypeResponse {
    pub fn new(uid: Uuid, name_ru: String, name_en: String) -> Self {
        Self { uid, name_ru, name_en }
    }

    pub fn from_dto(dto: &MaterialTypeDto) -> Self {
        Self {
            uid: dto.uid,
            name_ru: dto.name_ru.clone(),
            name_en: dto.name_en.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialNameResponse {
    pub uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

impl MaterialNameResponse {
    pub fn new(uid: Uuid, name_ru: String, name_en: String) -> Self {
        Self { uid, name_ru, name_en }
    }

    pub fn from_dto(dto: &MaterialNameDto) -> Self {
        Self {
            uid: dto.uid,
            name_ru: dto.name_ru.clone(),
            name_en: dto.name_en.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialTypesListResponse {
    pub data: Vec<MaterialTypeResponse>,
    pub total: usize,
}

impl MaterialTypesListResponse {
    pub fn new(data: Vec<MaterialTypeResponse>) -> Self {
        let total = data.len();
        Self { data, total }
    }

    pub fn from_dtos(dtos: Vec<MaterialTypeDto>) -> Self {
        let data = dtos.iter().map(MaterialTypeResponse::from_dto).collect();
        Self::new(data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialNamesListResponse {
    pub data: Vec<MaterialNameResponse>,
    pub total: usize,
}

impl MaterialNamesListResponse {
    pub fn new(data: Vec<MaterialNameResponse>) -> Self {
        let total = data.len();
        Self { data, total }
    }

    pub fn from_dtos(dtos: Vec<MaterialNameDto>) -> Self {
        let data = dtos.iter().map(MaterialNameResponse::from_dto).collect();
        Self::new(data)
    }
}
