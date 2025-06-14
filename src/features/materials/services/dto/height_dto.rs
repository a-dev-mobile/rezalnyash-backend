use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::features::materials::domain::{entities::Height, errors::MaterialError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeightDto {
    pub uid: Uuid,
    pub height: f64,
}

impl HeightDto {
    pub fn new(uid: Uuid, height: f64) -> Self {
        Self { uid, height }
    }

    pub fn from_domain(domain: &Height) -> Self {
        Self {
            uid: domain.id().value(),
            height: domain.height(),
        }
    }

    pub fn to_domain(&self) -> Result<Height, MaterialError> {
        use crate::features::materials::domain::value_objects::HeightUid;
        let uid = HeightUid::new(self.uid);
        Height::new(uid, self.height)
    }
}