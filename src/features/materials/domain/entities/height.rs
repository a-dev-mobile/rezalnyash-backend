use crate::features::materials::domain::{errors::MaterialError, value_objects::HeightUid};

#[derive(Debug, Clone, PartialEq)]
pub struct Height {
    id: HeightUid,
    height: f64,
}

impl Height {
    pub fn new(id: HeightUid, height: f64) -> Result<Self, MaterialError> {
        if height <= 0.0 {
            return Err(MaterialError::ValidationError {
                message: "Высота должна быть больше нуля".to_string(),
            });
        }

        Ok(Self { id, height })
    }

    pub fn create(height: f64) -> Result<Self, MaterialError> {
        if height <= 0.0 {
            return Err(MaterialError::ValidationError {
                message: "Высота должна быть больше нуля".to_string(),
            });
        }

        Ok(Self {
            id: HeightUid::generate(),
            height,
        })
    }

    pub fn id(&self) -> &HeightUid {
        &self.id
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn with_id(mut self, id: HeightUid) -> Self {
        self.id = id;
        self
    }
}