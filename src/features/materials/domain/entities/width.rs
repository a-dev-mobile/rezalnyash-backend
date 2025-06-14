use crate::features::materials::domain::{errors::MaterialError, value_objects::WidthUid};

#[derive(Debug, Clone, PartialEq)]
pub struct Width {
    id: WidthUid,
    width: f64,
}

impl Width {
    pub fn new(id: WidthUid, width: f64) -> Result<Self, MaterialError> {
        if width <= 0.0 {
            return Err(MaterialError::ValidationError {
                message: "Ширина должна быть больше нуля".to_string(),
            });
        }

        Ok(Self { id, width })
    }

    pub fn create(width: f64) -> Result<Self, MaterialError> {
        if width <= 0.0 {
            return Err(MaterialError::ValidationError {
                message: "Ширина должна быть больше нуля".to_string(),
            });
        }

        Ok(Self {
            id: WidthUid::generate(),
            width,
        })
    }

    pub fn id(&self) -> &WidthUid {
        &self.id
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn with_id(mut self, id: WidthUid) -> Self {
        self.id = id;
        self
    }
}