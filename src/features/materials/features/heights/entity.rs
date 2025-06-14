use uuid::Uuid;

use crate::features::materials::shared::errors::MaterialError;

/// Уникальный идентификатор высоты
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeightUid(Uuid);

impl HeightUid {
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

/// Доменная сущность высоты
#[derive(Debug, Clone, PartialEq)]
pub struct Height {
    id: HeightUid,
    value: f64,
}

impl Height {
    /// Создать новую высоту с проверкой
    pub fn new(value: f64) -> Result<Self, MaterialError> {
        if value <= 0.0 {
            return Err(MaterialError::ValidationError {
                message: "Высота должна быть больше нуля".to_string(),
            });
        }

        Ok(Self {
            id: HeightUid::new(),
            value,
        })
    }

    /// Создать высоту с существующим ID (из БД)
    pub fn from_db(id: HeightUid, value: f64) -> Result<Self, MaterialError> {
        if value <= 0.0 {
            return Err(MaterialError::ValidationError {
                message: "Высота должна быть больше нуля".to_string(),
            });
        }

        Ok(Self { id, value })
    }

    // Геттеры
    pub fn id(&self) -> HeightUid {
        self.id
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}