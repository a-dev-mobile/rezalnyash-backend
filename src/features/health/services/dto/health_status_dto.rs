use serde::{Deserialize, Serialize};
use crate::features::health::domain::entities::HealthStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatusDto {
    pub service_name: String,
    pub is_healthy: bool,
    pub message: Option<String>,
    pub timestamp: String,
}

impl HealthStatusDto {
    pub fn from_domain(domain: &HealthStatus) -> Self {
        Self {
            service_name: domain.service_name.clone(),
            is_healthy: domain.is_healthy,
            message: domain.message.clone(),
            timestamp: domain.timestamp.to_rfc3339(),
        }
    }
}