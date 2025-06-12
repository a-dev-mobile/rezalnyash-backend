
use std::sync::Arc;
use sqlx::{PgPool, Row};
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::materials::{Material, MaterialStandardSize, MaterialPreset, StandardSize, MaterialProperties};

/// Репозиторий для работы с материалами
#[derive(Debug, Clone)]
pub struct MaterialsRepository {
    pool: Arc<PgPool>,
}

impl MaterialsRepository {
    /// Создает новый экземпляр репозитория
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Получает все материалы с их стандартными размерами
    /// 
    /// # Возвращает
    /// 
    /// * `Result<Vec<MaterialPreset>, AppError>` - Список материалов или ошибка
    pub async fn get_all_materials(&self) -> Result<Vec<MaterialPreset>, AppError> {
        info!("Запрос всех материалов из базы данных");

        // Запрос материалов
        let materials_query = r#"
            SELECT 
                id, material_type, name_ru, name_en, 
                thickness, color, grain_direction, created_at
            FROM materials 
            ORDER BY material_type, name_ru
        "#;

        let materials: Vec<Material> = sqlx::query_as(materials_query)
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| {
                error!("Ошибка при получении материалов: {}", e);
                AppError::DatabaseError {
                    message: format!("Не удалось получить материалы: {}", e),
                }
            })?;

        debug!("Получено {} материалов", materials.len());

        // Получаем размеры для всех материалов одним запросом для оптимизации
        let sizes_query = r#"
            SELECT 
                material_id, width, height, name, common_usage
            FROM material_standard_sizes 
            ORDER BY material_id, width DESC, height DESC
        "#;

        let sizes: Vec<MaterialStandardSize> = sqlx::query_as(sizes_query)
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| {
                error!("Ошибка при получении стандартных размеров: {}", e);
                AppError::DatabaseError {
                    message: format!("Не удалось получить стандартные размеры: {}", e),
                }
            })?;

        debug!("Получено {} стандартных размеров", sizes.len());

        // Группируем размеры по material_id для быстрого поиска
        let mut sizes_by_material: std::collections::HashMap<Uuid, Vec<MaterialStandardSize>> = 
            std::collections::HashMap::new();
        
        for size in sizes {
            sizes_by_material
                .entry(size.material_id)
                .or_default()
                .push(size);
        }

        // Преобразуем в MaterialPreset
        let material_presets: Vec<MaterialPreset> = materials
            .into_iter()
            .map(|material| {
                let standard_sizes = sizes_by_material
                    .get(&material.id)
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|size| StandardSize {
                        width: size.width,
                        height: size.height,
                        name: size.name.clone(),
                        common_usage: size.common_usage.clone(),
                    })
                    .collect();

                // Получаем уникальные толщины для этого типа материала
                let default_thicknesses = self.get_default_thicknesses(&material.material_type);
                
                MaterialPreset {
                    material_type: material.material_type.clone(),
                    name_ru: material.name_ru,
                    name_en: material.name_en,
                    standard_sizes,
                    default_thicknesses,
                    properties: self.get_material_properties(&material.material_type, material.grain_direction),
                }
            })
            .collect();

        info!("Успешно сформировано {} материальных пресетов", material_presets.len());
        Ok(material_presets)
    }

    /// Получает стандартные толщины для типа материала
    fn get_default_thicknesses(&self, material_type: &str) -> Vec<f64> {
        match material_type {
            "ЛДСП" | "ДСП" => vec![8.0, 10.0, 16.0, 18.0, 22.0, 25.0, 28.0, 32.0],
            "МДФ" => vec![3.0, 6.0, 9.0, 12.0, 16.0, 18.0, 22.0, 25.0],
            "Фанера" => vec![4.0, 6.0, 9.0, 12.0, 15.0, 18.0, 21.0, 24.0, 27.0, 30.0],
            "ДВП" => vec![2.5, 3.2, 4.0, 5.0, 6.0],
            "Стекло" => vec![3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 12.0],
            "Металл" => vec![0.5, 0.8, 1.0, 1.2, 1.5, 2.0, 3.0, 4.0, 5.0],
            _ => vec![16.0], // По умолчанию
        }
    }

    /// Получает свойства материала
    fn get_material_properties(&self, material_type: &str, grain_direction: bool) -> MaterialProperties {
        match material_type {
            "ЛДСП" | "ДСП" => MaterialProperties {
                can_rotate: !grain_direction,
                has_grain: grain_direction,
                recommended_blade_width: 4.0,
                recommended_edge_margin: 8.0,
            },
            "МДФ" => MaterialProperties {
                can_rotate: true,
                has_grain: false,
                recommended_blade_width: 3.5,
                recommended_edge_margin: 6.0,
            },
            "Фанера" => MaterialProperties {
                can_rotate: !grain_direction,
                has_grain: grain_direction,
                recommended_blade_width: 3.0,
                recommended_edge_margin: 5.0,
            },
            "ДВП" => MaterialProperties {
                can_rotate: true,
                has_grain: false,
                recommended_blade_width: 2.5,
                recommended_edge_margin: 4.0,
            },
            "Стекло" => MaterialProperties {
                can_rotate: false, // Стекло обычно не поворачивают
                has_grain: false,
                recommended_blade_width: 1.0,
                recommended_edge_margin: 15.0, // Больший отступ для безопасности
            },
            "Металл" => MaterialProperties {
                can_rotate: true,
                has_grain: false,
                recommended_blade_width: 2.0,
                recommended_edge_margin: 10.0,
            },
            _ => MaterialProperties {
                can_rotate: true,
                has_grain: false,
                recommended_blade_width: 4.0,
                recommended_edge_margin: 10.0,
            },
        }
    }

    /// Проверяет доступность соединения с базой данных
    pub async fn health_check(&self) -> Result<(), AppError> {
        sqlx::query("SELECT 1")
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| AppError::DatabaseError {
                message: format!("Health check failed: {}", e),
            })?;
        Ok(())
    }
}
