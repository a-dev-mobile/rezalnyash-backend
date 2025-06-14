use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateThicknessDto {
    pub thickness: f64,
}

impl CreateThicknessDto {
    pub fn new(thickness: f64) -> Self {
        Self { thickness }
    }
}
