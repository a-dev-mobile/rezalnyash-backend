use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use crate::features::materials::shared::errors::MaterialError;
use super::entity::{MaterialType, MaterialTypeUid};

/// Модель БД для типа материала
#[derive(Debug, FromRow)]
pub struct MaterialTypeDb {
    pub material_type_uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

/// Трейт репозитория типов материалов
#[async_trait::async_trait]
pub trait TypeRepository: Send + Sync {
    async fn get_by_id(&self, id: MaterialTypeUid) -> Result<MaterialType, MaterialError>;
    async fn get_all(&self) -> Result<Vec<MaterialType>, MaterialError>;
    async fn create(&self, material_type: &MaterialType) -> Result<MaterialType, MaterialError>;
    async fn find_by_name(&self, name_ru: &str, name_en: &str) -> Result<Option<MaterialType>, MaterialError>;
    async fn exists(&self, id: MaterialTypeUid) -> Result<bool, MaterialError>;
}

/// PostgreSQL реализация репозитория
pub struct PostgresMaterialTypeRepository {
    pool: PgPool,
}

impl PostgresMaterialTypeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TypeRepository for PostgresMaterialTypeRepository {
    async fn get_by_id(&self, id: MaterialTypeUid) -> Result<MaterialType, MaterialError> {
        let row = sqlx::query_as::<_, MaterialTypeDb>(
            "SELECT material_type_uid, name_ru, name_en FROM materials.material_types WHERE material_type_uid = $1"
        )
        .bind(id.as_uuid())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => MaterialError::NotFoundError {
                message: format!("Тип материала с ID {} не найден", id.as_uuid()),
            },
            _ => MaterialError::DatabaseError {
                message: format!("Ошибка БД: {}", e),
            },
        })?;

        MaterialType::from_db(
            MaterialTypeUid::from_uuid(row.material_type_uid),
            row.name_ru,
            row.name_en,
        )
    }

    async fn get_all(&self) -> Result<Vec<MaterialType>, MaterialError> {
        let rows = sqlx::query_as::<_, MaterialTypeDb>(
            "SELECT material_type_uid, name_ru, name_en FROM materials.material_types ORDER BY name_ru"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка получения типов материалов: {}", e),
        })?;

        rows.into_iter()
            .map(|row| MaterialType::from_db(
                MaterialTypeUid::from_uuid(row.material_type_uid),
                row.name_ru,
                row.name_en,
            ))
            .collect()
    }

    async fn create(&self, material_type: &MaterialType) -> Result<MaterialType, MaterialError> {
        // Проверяем дубликаты
        if let Ok(Some(_)) = self.find_by_name(material_type.name_ru(), material_type.name_en()).await {
            return Err(MaterialError::DuplicateError {
                message: format!("Тип материала с названиями '{}' / '{}' уже существует", 
                    material_type.name_ru(), material_type.name_en()),
            });
        }

        let row = sqlx::query_as::<_, MaterialTypeDb>(
            "INSERT INTO materials.material_types (material_type_uid, name_ru, name_en) 
             VALUES ($1, $2, $3) 
             RETURNING material_type_uid, name_ru, name_en"
        )
        .bind(material_type.id().as_uuid())
        .bind(material_type.name_ru())
        .bind(material_type.name_en())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка создания типа материала: {}", e),
        })?;

        MaterialType::from_db(
            MaterialTypeUid::from_uuid(row.material_type_uid),
            row.name_ru,
            row.name_en,
        )
    }

    async fn find_by_name(&self, name_ru: &str, name_en: &str) -> Result<Option<MaterialType>, MaterialError> {
        let row = sqlx::query_as::<_, MaterialTypeDb>(
            "SELECT material_type_uid, name_ru, name_en FROM materials.material_types 
             WHERE name_ru = $1 AND name_en = $2"
        )
        .bind(name_ru)
        .bind(name_en)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка поиска типа материала: {}", e),
        })?;

        match row {
            Some(row) => Ok(Some(MaterialType::from_db(
                MaterialTypeUid::from_uuid(row.material_type_uid),
                row.name_ru,
                row.name_en,
            )?)),
            None => Ok(None),
        }
    }

    async fn exists(&self, id: MaterialTypeUid) -> Result<bool, MaterialError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM materials.material_types WHERE material_type_uid = $1"
        )
        .bind(id.as_uuid())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка проверки существования: {}", e),
        })?;

        Ok(count > 0)
    }
}