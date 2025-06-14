use uuid::Uuid;

use crate::features::materials::shared::errors::MaterialError;


/// Уникальный идентификатор ширины
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidthUid(Uuid);

impl WidthUid {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
    
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

/// Доменная сущность ширины
#[derive(Debug, Clone, PartialEq)]
pub struct Width {
    id: WidthUid,
    value: f64,
}

impl Width {
    /// Создать новую ширину с проверкой
    pub fn new(value: f64) -> Result<Self, MaterialError> {
        if value <= 0.0 {
            return Err(MaterialError::ValidationError {
                message: "Ширина должна быть больше нуля".to_string(),
            });
        }

        Ok(Self {
            id: WidthUid::new(),
            value,
        })
    }

    /// Создать ширину с существующим ID (из БД)
    pub fn from_db(id: WidthUid, value: f64) -> Result<Self, MaterialError> {
        if value <= 0.0 {
            return Err(MaterialError::ValidationError {
                message: "Ширина должна быть больше нуля".to_string(),
            });
        }

        Ok(Self { id, value })
    }

    // Геттеры
    pub fn id(&self) -> WidthUid {
        self.id
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
