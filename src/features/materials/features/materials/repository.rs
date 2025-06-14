// src/features/materials/features/materials/repository.rs
use sqlx::{PgPool, FromRow, Row};
use uuid::Uuid;
use std::collections::HashMap;
use crate::features::materials::shared::errors::MaterialError;
use super::entity::{Material, MaterialUid, StandardSize};

/// Модель БД для материала с объединенными данными
#[derive(Debug, FromRow)]
pub struct MaterialDb {
    pub material_uid: Uuid,
    pub type_name_ru: String,
    pub type_name_en: String,
    pub name_ru: String,
    pub name_en: String,
}

/// Модель БД для размеров
#[derive(Debug, FromRow)]
pub struct MaterialSizeDb {
    pub material_uid: Uuid,
    pub width: f64,
    pub height: f64,
}

/// Модель БД для толщин
#[derive(Debug, FromRow)]
pub struct MaterialThicknessDb {
    pub material_uid: Uuid,
    pub thickness: f64,
}

/// Трейт репозитория материалов
#[async_trait::async_trait]
pub trait MaterialRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Material>, MaterialError>;
}

/// PostgreSQL реализация репозитория
pub struct PostgresMaterialRepository {
    pool: PgPool,
}

impl PostgresMaterialRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl MaterialRepository for PostgresMaterialRepository {
    async fn get_all(&self) -> Result<Vec<Material>, MaterialError> {
        // Получаем базовую информацию о материалах
        let materials_query = r#"
            SELECT DISTINCT
                m.material_uid,
                t.name_ru as type_name_ru,
                t.name_en as type_name_en,
                n.name_ru,
                n.name_en
            FROM materials.materials m
            JOIN materials.types t ON m.type_uid = t.type_uid
            JOIN materials.names n ON m.name_uid = n.name_uid
            ORDER BY t.name_ru, n.name_ru
        "#;

        let material_rows = sqlx::query_as::<_, MaterialDb>(materials_query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| MaterialError::DatabaseError {
                message: format!("Ошибка получения материалов: {}", e),
            })?;

        if material_rows.is_empty() {
            return Ok(vec![]);
        }

        // Получаем все размеры для всех материалов
        let sizes_query = r#"
            SELECT 
                m.material_uid,
                w.width,
                h.height
            FROM materials.materials m
            JOIN materials.widths w ON m.width_uid = w.width_uid
            JOIN materials.heights h ON m.height_uid = h.height_uid
            ORDER BY m.material_uid, w.width, h.height
        "#;

        let size_rows = sqlx::query_as::<_, MaterialSizeDb>(sizes_query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| MaterialError::DatabaseError {
                message: format!("Ошибка получения размеров материалов: {}", e),
            })?;

        // Получаем все толщины для всех материалов
        let thicknesses_query = r#"
            SELECT 
                m.material_uid,
                th.thickness
            FROM materials.materials m
            JOIN materials.thicknesses th ON m.thickness_uid = th.thickness_uid
            ORDER BY m.material_uid, th.thickness
        "#;

        let thickness_rows = sqlx::query_as::<_, MaterialThicknessDb>(thicknesses_query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| MaterialError::DatabaseError {
                message: format!("Ошибка получения толщин материалов: {}", e),
            })?;

        // Группируем размеры по material_uid
        let mut sizes_map: HashMap<Uuid, Vec<StandardSize>> = HashMap::new();
        for size_row in size_rows {
            sizes_map
                .entry(size_row.material_uid)
                .or_insert_with(Vec::new)
                .push(StandardSize::new(size_row.width, size_row.height));
        }

        // Группируем толщины по material_uid
        let mut thicknesses_map: HashMap<Uuid, Vec<f64>> = HashMap::new();
        for thickness_row in thickness_rows {
            thicknesses_map
                .entry(thickness_row.material_uid)
                .or_insert_with(Vec::new)
                .push(thickness_row.thickness);
        }

        // Создаем материалы, объединяя данные
        let mut materials = Vec::new();
        for material_row in material_rows {
            let standard_sizes = sizes_map
                .get(&material_row.material_uid)
                .cloned()
                .unwrap_or_default();

            let default_thicknesses = thicknesses_map
                .get(&material_row.material_uid)
                .cloned()
                .unwrap_or_default();

            let material = Material::from_db(
                MaterialUid::from_uuid(material_row.material_uid),
                material_row.type_name_ru,
                material_row.type_name_en,
                material_row.name_ru,
                material_row.name_en,
                standard_sizes,
                default_thicknesses,
            );

            materials.push(material);
        }

        Ok(materials)
    }
}