use crate::features::materials::domain::value_objects::ThicknessUid;
use crate::features::materials::domain::{entities::Thickness, errors::MaterialError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThicknessDto {
    pub uid: Uuid,
    pub thickness: f64,
}

impl ThicknessDto {
    pub fn new(uid: Uuid, thickness: f64) -> Self {
        Self { uid, thickness }
    }

    pub fn from_domain(domain: &Thickness) -> Self {
        Self {
            uid: domain.id().value(),
            thickness: domain.thickness(),
        }
    }

    pub fn to_domain(&self) -> Result<Thickness, MaterialError> {
        let uid = ThicknessUid::new(self.uid);
        Thickness::new(uid, self.thickness)
    }
}
