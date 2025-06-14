use crate::features::health::domain::errors::HealthError;

#[derive(Debug, Clone, PartialEq)]
pub struct HealthStatus {
    pub service_name: String,
    pub is_healthy: bool,
    pub message: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl HealthStatus {
    pub fn healthy(service_name: String) -> Self {
        Self {
            service_name,
            is_healthy: true,
            message: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn unhealthy(service_name: String, message: String) -> Self {
        Self {
            service_name,
            is_healthy: false,
            message: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }
}