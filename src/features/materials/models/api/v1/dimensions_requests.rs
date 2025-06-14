use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWidthRequest {
    pub width: f64,
}

impl CreateWidthRequest {
    pub fn new(width: f64) -> Self {
        Self { width }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHeightRequest {
    pub height: f64,
}

impl CreateHeightRequest {
    pub fn new(height: f64) -> Self {
        Self { height }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateThicknessRequest {
    pub thickness: f64,
}

impl CreateThicknessRequest {
    pub fn new(thickness: f64) -> Self {
        Self { thickness }
    }
}