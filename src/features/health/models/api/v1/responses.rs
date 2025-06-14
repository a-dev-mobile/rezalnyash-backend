use crate::features::health::services::dto::HealthStatusDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub service_name: String,
    pub is_healthy: bool,
    pub message: Option<String>,
    pub timestamp: String,
}

impl HealthResponse {
    pub fn from_dto(dto: &HealthStatusDto) -> Self {
        Self {
            service_name: dto.service_name.clone(),
            is_healthy: dto.is_healthy,
            message: dto.message.clone(),
            timestamp: dto.timestamp.clone(),
        }
    }
}