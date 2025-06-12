
use std::sync::Arc;
use tracing::{debug, error, info, warn};

use crate::database::repositories::materials::MaterialsRepository;
use crate::error::AppError;
use crate::models::materials::{MaterialPreset, MaterialsResponse};

/// Сервис для работы с материалами
/// 
/// Обеспечивает бизнес-логику для операций с материалами,
/// включая кэширование, валидацию и обработку ошибок.
#[derive(Debug, Clone)]
pub struct MaterialsService {
    repository: Arc<MaterialsRepository>,
}

impl MaterialsService {
    /// Создает новый экземпляр сервиса материалов
    /// 
    /// # Аргументы
    /// 
    /// * `repository` - Репозиторий для работы с данными материалов
    /// 
    /// # Возвращает
    /// 
    /// Новый экземпляр `MaterialsService`
    pub fn new(repository: Arc<MaterialsRepository>) -> Self {
        info!("Инициализация MaterialsService");
        Self { repository }
    }

    /// Получает все доступные материалы
    /// 
    /// Возвращает список всех материалов с их стандартными размерами,
    /// толщинами и свойствами. Результат может быть кэширован для
    /// повышения производительности.
    /// 
    /// # Возвращает
    /// 
    /// * `Result<MaterialsResponse, AppError>` - Ответ со списком материалов или ошибка
    /// 
    /// # Ошибки
    /// 
    /// Возвращает `AppError::DatabaseError` если не удалось получить данные из БД
    pub async fn get_all_materials(&self) -> Result<MaterialsResponse, AppError> {
        info!("Запрос всех материалов через MaterialsService");
        
        // Получаем материалы из репозитория
        let materials = self.repository.get_all_materials().await?;
        
        // Проверяем, что получили данные
        if materials.is_empty() {
            warn!("В базе данных не найдено ни одного материала");
            // Можно вернуть ошибку или пустой список - зависит от бизнес-требований
            // Пока возвращаем пустой список
        } else {
            info!("Получено {} материалов", materials.len());
        }

        // Дополнительная валидация данных
        let validated_materials = self.validate_materials(materials)?;

        Ok(MaterialsResponse { 
            materials: validated_materials 
        })
    }

    /// Валидирует полученные материалы
    /// 
    /// Проверяет корректность данных материалов и исправляет
    /// потенциальные проблемы.
    /// 
    /// # Аргументы
    /// 
    /// * `materials` - Список материалов для валидации
    /// 
    /// # Возвращает
    /// 
    /// * `Result<Vec<MaterialPreset>, AppError>` - Валидированный список материалов
    fn validate_materials(&self, materials: Vec<MaterialPreset>) -> Result<Vec<MaterialPreset>, AppError> {
        debug!("Валидация {} материалов", materials.len());
        
        let mut validated = Vec::new();
        let mut validation_errors = Vec::new();

        for material in materials {
            match self.validate_single_material(&material) {
                Ok(validated_material) => validated.push(validated_material),
                Err(e) => {
                    error!("Ошибка валидации материала {}: {}", material.material_type, e);
                    validation_errors.push(format!("Материал '{}': {}", material.material_type, e));
                }
            }
        }

        // Если есть критичные ошибки валидации, возвращаем ошибку
        if !validation_errors.is_empty() && validated.is_empty() {
            return Err(AppError::ValidationError {
                field: "materials".to_string(),
                message: format!("Критичные ошибки валидации: {}", validation_errors.join("; ")),
            });
        }

        // Логируем предупреждения, но продолжаем работу
        if !validation_errors.is_empty() {
            warn!("Обнаружены ошибки валидации (не критичные): {:?}", validation_errors);
        }

        debug!("Валидация завершена, {} материалов прошли проверку", validated.len());
        Ok(validated)
    }

    /// Валидирует отдельный материал
    fn validate_single_material(&self, material: &MaterialPreset) -> Result<MaterialPreset, AppError> {
        // Проверяем обязательные поля
        if material.material_type.trim().is_empty() {
            return Err(AppError::ValidationError {
                field: "material_type".to_string(),
                message: "Тип материала не может быть пустым".to_string(),
            });
        }

        if material.name_ru.trim().is_empty() {
            return Err(AppError::ValidationError {
                field: "name_ru".to_string(),
                message: "Русское название материала не может быть пустым".to_string(),
            });
        }

        // Проверяем стандартные размеры
        for (index, size) in material.standard_sizes.iter().enumerate() {
            if size.width <= 0.0 || size.height <= 0.0 {
                return Err(AppError::ValidationError {
                    field: format!("standard_sizes[{}]", index),
                    message: format!("Некорректные размеры: {}x{}", size.width, size.height),
                });
            }

            // Проверяем разумные пределы размеров (от 10мм до 10м)
            if size.width < 10.0 || size.width > 10000.0 || 
               size.height < 10.0 || size.height > 10000.0 {
                warn!("Подозрительные размеры для материала {}: {}x{}", 
                      material.material_type, size.width, size.height);
            }
        }

        // Проверяем толщины
        for (index, thickness) in material.default_thicknesses.iter().enumerate() {
            if *thickness <= 0.0 {
                return Err(AppError::ValidationError {
                    field: format!("default_thicknesses[{}]", index),
                    message: format!("Некорректная толщина: {}", thickness),
                });
            }
        }

        // Проверяем свойства материала
        if material.properties.recommended_blade_width < 0.0 {
            return Err(AppError::ValidationError {
                field: "properties.recommended_blade_width".to_string(),
                message: "Ширина пропила не может быть отрицательной".to_string(),
            });
        }

        if material.properties.recommended_edge_margin < 0.0 {
            return Err(AppError::ValidationError {
                field: "properties.recommended_edge_margin".to_string(),
                message: "Отступ от края не может быть отрицательным".to_string(),
            });
        }

        // Если все проверки прошли, возвращаем клон материала
        Ok(material.clone())
    }

    /// Проверяет работоспособность сервиса
    pub async fn health_check(&self) -> Result<(), AppError> {
        debug!("Проверка работоспособности MaterialsService");
        self.repository.health_check().await?;
        debug!("MaterialsService работает корректно");
        Ok(())
    }
}