use uuid::Uuid;
use crate::features::materials::shared::errors::MaterialError;

/// Уникальный идентификатор толщины
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThicknessUid(Uuid);

impl ThicknessUid {
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

/// Доменная сущность толщины
#[derive(Debug, Clone, PartialEq)]
pub struct Thickness {
    id: ThicknessUid,
    value: f64,
}

impl Thickness {
    /// Создать новую толщину с проверкой
    pub fn new(value: f64) -> Result<Self, MaterialError> {
        if value <= 0.0 {
            return Err(MaterialError::ValidationError {
                message: "Толщина должна быть больше нуля".to_string(),
            });
        }

        Ok(Self {
            id: ThicknessUid::new(),
            value,
        })
    }

    /// Создать толщину с существующим ID (из БД)
    pub fn from_db(id: ThicknessUid, value: f64) -> Result<Self, MaterialError> {
        if value <= 0.0 {
            return Err(MaterialError::ValidationError {
                message: "Толщина должна быть больше нуля".to_string(),
            });
        }

        Ok(Self { id, value })
    }

    // Геттеры
    pub fn id(&self) -> ThicknessUid {
        self.id
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}