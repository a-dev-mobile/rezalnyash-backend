use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ThicknessDb {
    pub thickness_uid: Uuid,
    pub thickness: f64,
}

impl ThicknessDb {
    pub fn new(thickness_uid: Uuid, thickness: f64) -> Self {
        Self {
            thickness_uid,
            thickness,
        }
    }
}