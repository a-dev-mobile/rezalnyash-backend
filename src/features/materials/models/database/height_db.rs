use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct HeightDb {
    pub height_uid: Uuid,
    pub height: f64,
}

impl HeightDb {
    pub fn new(height_uid: Uuid, height: f64) -> Self {
        Self {
            height_uid,
            height,
        }
    }
}
