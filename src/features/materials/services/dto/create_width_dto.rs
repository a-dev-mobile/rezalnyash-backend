use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWidthDto {
    pub width: f64,
}

impl CreateWidthDto {
    pub fn new(width: f64) -> Self {
        Self { width }
    }
}