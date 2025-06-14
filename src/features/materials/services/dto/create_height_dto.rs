use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHeightDto {
    pub height: f64,
}

impl CreateHeightDto {
    pub fn new(height: f64) -> Self {
        Self { height }
    }
}