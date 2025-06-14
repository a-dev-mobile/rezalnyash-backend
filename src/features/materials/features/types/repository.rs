use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use crate::features::materials::shared::errors::MaterialError;
use super::entity::{Type, TypeUid};

/// Модель БД для типа материала
#[derive(Debug, FromRow)]
pub struct TypeDb {
    pub material_type_uid: Uuid,
    pub name_ru: String,
    pub name_en: String,
}

/// Трейт репозитория типов материалов
#[async_trait::async_trait]
pub trait TypeRepository: Send + Sync {
    async fn get_by_id(&self, id: TypeUid) -> Result<Type, MaterialError>;
    async fn get_all(&self) -> Result<Vec<Type>, MaterialError>;
    async fn create(&self, material_type: &Type) -> Result<Type, MaterialError>;
    async fn find_by_name(&self, name_ru: &str, name_en: &str) -> Result<Option<Type>, MaterialError>;
    async fn exists(&self, id: TypeUid) -> Result<bool, MaterialError>;
}

/// PostgreSQL реализация репозитория
pub struct PostgresTypeRepository {
    pool: PgPool,
}

impl PostgresTypeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TypeRepository for PostgresTypeRepository {
    async fn get_by_id(&self, id: TypeUid) -> Result<Type, MaterialError> {
        let row = sqlx::query_as::<_, TypeDb>(
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

        Type::from_db(
            TypeUid::from_uuid(row.material_type_uid),
            row.name_ru,
            row.name_en,
        )
    }

    async fn get_all(&self) -> Result<Vec<Type>, MaterialError> {
        let rows = sqlx::query_as::<_, TypeDb>(
            "SELECT material_type_uid, name_ru, name_en FROM materials.material_types ORDER BY name_ru"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка получения типов материалов: {}", e),
        })?;

        rows.into_iter()
            .map(|row| Type::from_db(
                TypeUid::from_uuid(row.material_type_uid),
                row.name_ru,
                row.name_en,
            ))
            .collect()
    }

    async fn create(&self, material_type: &Type) -> Result<Type, MaterialError> {
        // Проверяем дубликаты
        if let Ok(Some(_)) = self.find_by_name(material_type.name_ru(), material_type.name_en()).await {
            return Err(MaterialError::DuplicateError {
                message: format!("Тип материала с названиями '{}' / '{}' уже существует", 
                    material_type.name_ru(), material_type.name_en()),
            });
        }

        let row = sqlx::query_as::<_, TypeDb>(
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

        Type::from_db(
            TypeUid::from_uuid(row.material_type_uid),
            row.name_ru,
            row.name_en,
        )
    }

    async fn find_by_name(&self, name_ru: &str, name_en: &str) -> Result<Option<Type>, MaterialError> {
        let row = sqlx::query_as::<_, TypeDb>(
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
            Some(row) => Ok(Some(Type::from_db(
                TypeUid::from_uuid(row.material_type_uid),
                row.name_ru,
                row.name_en,
            )?)),
            None => Ok(None),
        }
    }

    async fn exists(&self, id: TypeUid) -> Result<bool, MaterialError> {
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