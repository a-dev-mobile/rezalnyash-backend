use uuid::Uuid;
use crate::features::materials::shared::errors::MaterialError;

/// Уникальный идентификатор названия материала
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialNameUid(Uuid);

impl MaterialNameUid {
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

/// Доменная сущность названия материала
#[derive(Debug, Clone, PartialEq)]
pub struct MaterialName {
    id: MaterialNameUid,
    name_ru: String,
    name_en: String,
}

impl MaterialName {
    /// Создать новое название материала с проверкой
    pub fn new(name_ru: String, name_en: String) -> Result<Self, MaterialError> {
        if name_ru.trim().is_empty() {
            return Err(MaterialError::ValidationError {
                message: "Название материала (RU) не может быть пустым".to_string(),
            });
        }
        
        if name_en.trim().is_empty() {
            return Err(MaterialError::ValidationError {
                message: "Название материала (EN) не может быть пустым".to_string(),
            });
        }

        Ok(Self {
            id: MaterialNameUid::new(),
            name_ru: name_ru.trim().to_string(),
            name_en: name_en.trim().to_string(),
        })
    }

    /// Создать название материала с существующим ID (из БД)
    pub fn from_db(id: MaterialNameUid, name_ru: String, name_en: String) -> Result<Self, MaterialError> {
        if name_ru.trim().is_empty() {
            return Err(MaterialError::ValidationError {
                message: "Название материала (RU) не может быть пустым".to_string(),
            });
        }
        
        if name_en.trim().is_empty() {
            return Err(MaterialError::ValidationError {
                message: "Название материала (EN) не может быть пустым".to_string(),
            });
        }

        Ok(Self { 
            id, 
            name_ru: name_ru.trim().to_string(),
            name_en: name_en.trim().to_string(),
        })
    }

    // Геттеры
    pub fn id(&self) -> MaterialNameUid {
        self.id
    }

    pub fn name_ru(&self) -> &str {
        &self.name_ru
    }

    pub fn name_en(&self) -> &str {
        &self.name_en
    }
}
