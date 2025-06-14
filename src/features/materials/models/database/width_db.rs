use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WidthDb {
    pub width_uid: Uuid,
    pub width: f64,
}

impl WidthDb {
    pub fn new(width_uid: Uuid, width: f64) -> Self {
        Self {
            width_uid,
            width,
        }
    }
}