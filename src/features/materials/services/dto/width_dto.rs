use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::features::materials::domain::{entities::Width, errors::MaterialError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidthDto {
    pub uid: Uuid,
    pub width: f64,
}

impl WidthDto {
    pub fn new(uid: Uuid, width: f64) -> Self {
        Self { uid, width }
    }

    pub fn from_domain(domain: &Width) -> Self {
        Self {
            uid: domain.id().value(),
            width: domain.width(),
        }
    }

    pub fn to_domain(&self) -> Result<Width, MaterialError> {
        use crate::features::materials::domain::value_objects::WidthUid;
        let uid = WidthUid::new(self.uid);
        Width::new(uid, self.width)
    }
}