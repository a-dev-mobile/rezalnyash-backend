use rand::seq::index;

use crate::features::materials::domain::errors::MaterialError;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialTypeId {
    value: i32,
}

impl MaterialTypeId {
    pub fn new(value: i32) -> Result<Self, MaterialError> {
        if value <= 0 {
            return Err(MaterialError::ValidationError {
                message: "MaterialTypeId must be greater than 0".to_string(),
            });
        }
        Ok(Self { value })
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

impl std::fmt::Display for MaterialTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
