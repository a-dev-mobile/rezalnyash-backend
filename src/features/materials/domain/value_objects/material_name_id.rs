use crate::features::materials::domain::errors::MaterialError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialNameId {
    value: i32,
}

impl MaterialNameId {
    pub fn new(value: i32) -> Result<Self, MaterialError> {
        if value <= 0 {
            return Err(MaterialError::ValidationError {
                message: "MaterialNameId must be greater than 0".to_string(),
            });
        }
        Ok(Self { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

impl std::fmt::Display for MaterialNameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
