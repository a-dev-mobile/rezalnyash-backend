use crate::features::materials::domain::{errors::MaterialError, value_objects::MaterialNameUid};

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialName {
    id: MaterialNameUid,
    name_ru: String,
    name_en: String,
}

impl MaterialName {
    pub fn new(id: MaterialNameUid, name_ru: String, name_en: String) -> Result<Self, MaterialError> {
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

    // Создание новой сущности без ID (для создания)
    pub fn create(name_ru: String, name_en: String) -> Result<Self, MaterialError> {
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
            id: MaterialNameUid::generate(), // Генерируем новый UUID
            name_ru: name_ru.trim().to_string(),
            name_en: name_en.trim().to_string(),
        })
    }

    pub fn id(&self) -> &MaterialNameUid {
        &self.id
    }

    pub fn name_ru(&self) -> &str {
        &self.name_ru
    }

    pub fn name_en(&self) -> &str {
        &self.name_en
    }

    // Для установки ID после сохранения в БД
    pub fn with_id(mut self, id: MaterialNameUid) -> Self {
        self.id = id;
        self
    }
}
