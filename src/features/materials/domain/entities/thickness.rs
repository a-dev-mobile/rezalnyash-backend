use crate::features::materials::domain::{errors::MaterialError, value_objects::ThicknessUid};

#[derive(Debug, Clone, PartialEq)]
pub struct Thickness {
    id: ThicknessUid,
    thickness: f64,
}

impl Thickness {
    pub fn new(id: ThicknessUid, thickness: f64) -> Result<Self, MaterialError> {
        if thickness <= 0.0 {
            return Err(MaterialError::ValidationError {
                message: "Толщина должна быть больше нуля".to_string(),
            });
        }

        Ok(Self { id, thickness })
    }

    pub fn create(thickness: f64) -> Result<Self, MaterialError> {
        if thickness <= 0.0 {
            return Err(MaterialError::ValidationError {
                message: "Толщина должна быть больше нуля".to_string(),
            });
        }

        Ok(Self {
            id: ThicknessUid::generate(),
            thickness,
        })
    }

    pub fn id(&self) -> &ThicknessUid {
        &self.id
    }

    pub fn thickness(&self) -> f64 {
        self.thickness
    }

    pub fn with_id(mut self, id: ThicknessUid) -> Self {
        self.id = id;
        self
    }
}