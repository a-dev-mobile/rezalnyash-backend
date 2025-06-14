use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::features::materials::services::dto::{WidthDto, HeightDto, ThicknessDto};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidthResponse {
    pub uid: Uuid,
    pub width: f64,
}

impl WidthResponse {
    pub fn new(uid: Uuid, width: f64) -> Self {
        Self { uid, width }
    }

    pub fn from_dto(dto: &WidthDto) -> Self {
        Self {
            uid: dto.uid,
            width: dto.width,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidthsListResponse {
    pub data: Vec<WidthResponse>,
    pub total: usize,
}

impl WidthsListResponse {
    pub fn new(data: Vec<WidthResponse>) -> Self {
        let total = data.len();
        Self { data, total }
    }

    pub fn from_dtos(dtos: Vec<WidthDto>) -> Self {
        let data = dtos.iter().map(WidthResponse::from_dto).collect();
        Self::new(data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeightResponse {
    pub uid: Uuid,
    pub height: f64,
}

impl HeightResponse {
    pub fn new(uid: Uuid, height: f64) -> Self {
        Self { uid, height }
    }

    pub fn from_dto(dto: &HeightDto) -> Self {
        Self {
            uid: dto.uid,
            height: dto.height,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeightsListResponse {
    pub data: Vec<HeightResponse>,
    pub total: usize,
}

impl HeightsListResponse {
    pub fn new(data: Vec<HeightResponse>) -> Self {
        let total = data.len();
        Self { data, total }
    }

    pub fn from_dtos(dtos: Vec<HeightDto>) -> Self {
        let data = dtos.iter().map(HeightResponse::from_dto).collect();
        Self::new(data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThicknessResponse {
    pub uid: Uuid,
    pub thickness: f64,
}

impl ThicknessResponse {
    pub fn new(uid: Uuid, thickness: f64) -> Self {
        Self { uid, thickness }
    }

    pub fn from_dto(dto: &ThicknessDto) -> Self {
        Self {
            uid: dto.uid,
            thickness: dto.thickness,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThicknessesListResponse {
    pub data: Vec<ThicknessResponse>,
    pub total: usize,
}

impl ThicknessesListResponse {
    pub fn new(data: Vec<ThicknessResponse>) -> Self {
        let total = data.len();
        Self { data, total }
    }

    pub fn from_dtos(dtos: Vec<ThicknessDto>) -> Self {
        let data = dtos.iter().map(ThicknessResponse::from_dto).collect();
        Self::new(data)
    }
}