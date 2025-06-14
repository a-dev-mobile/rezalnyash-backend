use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use crate::features::materials::shared::errors::MaterialError;
use super::entity::{MaterialName, MaterialNameUid};

/// Модель БД для названия материала
#[derive(Debug, FromRow)]
pub struct MaterialNameDb {
    pub material_name_uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

/// Трейт репозитория названий материалов
#[async_trait::async_trait]
pub trait MaterialNameRepository: Send + Sync {
    async fn get_by_id(&self, id: MaterialNameUid) -> Result<MaterialName, MaterialError>;
    async fn get_all(&self) -> Result<Vec<MaterialName>, MaterialError>;
    async fn create(&self, material_name: &MaterialName) -> Result<MaterialName, MaterialError>;
    async fn find_by_name(&self, name_ru: &str, name_en: &str) -> Result<Option<MaterialName>, MaterialError>;
    async fn exists(&self, id: MaterialNameUid) -> Result<bool, MaterialError>;
}

/// PostgreSQL реализация репозитория
pub struct PostgresMaterialNameRepository {
    pool: PgPool,
}

impl PostgresMaterialNameRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl MaterialNameRepository for PostgresMaterialNameRepository {
    async fn get_by_id(&self, id: MaterialNameUid) -> Result<MaterialName, MaterialError> {
        let row = sqlx::query_as::<_, MaterialNameDb>(
            "SELECT material_name_uid, name_ru, name_en FROM materials.material_names WHERE material_name_uid = $1"
        )
        .bind(id.as_uuid())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => MaterialError::NotFoundError {
                message: format!("Название материала с ID {} не найдено", id.as_uuid()),
            },
            _ => MaterialError::DatabaseError {
                message: format!("Ошибка БД: {}", e),
            },
        })?;

        MaterialName::from_db(
            MaterialNameUid::from_uuid(row.material_name_uid),
            row.name_ru,
            row.name_en,
        )
    }

    async fn get_all(&self) -> Result<Vec<MaterialName>, MaterialError> {
        let rows = sqlx::query_as::<_, MaterialNameDb>(
            "SELECT material_name_uid, name_ru, name_en FROM materials.material_names ORDER BY name_ru"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка получения названий материалов: {}", e),
        })?;

        rows.into_iter()
            .map(|row| MaterialName::from_db(
                MaterialNameUid::from_uuid(row.material_name_uid),
                row.name_ru,
                row.name_en,
            ))
            .collect()
    }

    async fn create(&self, material_name: &MaterialName) -> Result<MaterialName, MaterialError> {
        // Проверяем дубликаты
        if let Ok(Some(_)) = self.find_by_name(material_name.name_ru(), material_name.name_en()).await {
            return Err(MaterialError::DuplicateError {
                message: format!("Название материала '{}' / '{}' уже существует", 
                    material_name.name_ru(), material_name.name_en()),
            });
        }

        let row = sqlx::query_as::<_, MaterialNameDb>(
            "INSERT INTO materials.material_names (material_name_uid, name_ru, name_en) 
             VALUES ($1, $2, $3) 
             RETURNING material_name_uid, name_ru, name_en"
        )
        .bind(material_name.id().as_uuid())
        .bind(material_name.name_ru())
        .bind(material_name.name_en())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка создания названия материала: {}", e),
        })?;

        MaterialName::from_db(
            MaterialNameUid::from_uuid(row.material_name_uid),
            row.name_ru,
            row.name_en,
        )
    }

    async fn find_by_name(&self, name_ru: &str, name_en: &str) -> Result<Option<MaterialName>, MaterialError> {
        let row = sqlx::query_as::<_, MaterialNameDb>(
            "SELECT material_name_uid, name_ru, name_en FROM materials.material_names 
             WHERE name_ru = $1 AND name_en = $2"
        )
        .bind(name_ru)
        .bind(name_en)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка поиска названия материала: {}", e),
        })?;

        match row {
            Some(row) => Ok(Some(MaterialName::from_db(
                MaterialNameUid::from_uuid(row.material_name_uid),
                row.name_ru,
                row.name_en,
            )?)),
            None => Ok(None),
        }
    }

    async fn exists(&self, id: MaterialNameUid) -> Result<bool, MaterialError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM materials.material_names WHERE material_name_uid = $1"
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