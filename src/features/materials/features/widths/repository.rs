// widths/repository.rs - Репозиторий + БД модель

use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::features::materials::shared::errors::MaterialError;

use super::entity::{Width, WidthUid};

/// Модель БД для ширины
#[derive(Debug, FromRow)]
pub struct WidthDb {
    pub width_uid: Uuid,
    pub width: f64,
}

/// Трейт репозитория ширин
#[async_trait::async_trait]
pub trait WidthRepository: Send + Sync {
    async fn get_by_id(&self, id: WidthUid) -> Result<Width, MaterialError>;
    async fn get_all(&self) -> Result<Vec<Width>, MaterialError>;
    async fn create(&self, width: &Width) -> Result<Width, MaterialError>;
    async fn find_by_value(&self, value: f64) -> Result<Option<Width>, MaterialError>;
    async fn exists(&self, id: WidthUid) -> Result<bool, MaterialError>;
}

/// PostgreSQL реализация репозитория
pub struct PostgresWidthRepository {
    pool: PgPool,
}

impl PostgresWidthRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl WidthRepository for PostgresWidthRepository {
    async fn get_by_id(&self, id: WidthUid) -> Result<Width, MaterialError> {
        let row = sqlx::query_as::<_, WidthDb>(
            "SELECT width_uid, width FROM materials.widths WHERE width_uid = $1"
        )
        .bind(id.as_uuid())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => MaterialError::NotFoundError {
                message: format!("Ширина с ID {} не найдена", id.as_uuid()),
            },
            _ => MaterialError::DatabaseError {
                message: format!("Ошибка БД: {}", e),
            },
        })?;

        Width::from_db(WidthUid::from_uuid(row.width_uid), row.width)
    }

    async fn get_all(&self) -> Result<Vec<Width>, MaterialError> {
        let rows = sqlx::query_as::<_, WidthDb>(
            "SELECT width_uid, width FROM materials.widths ORDER BY width"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка получения ширин: {}", e),
        })?;

        rows.into_iter()
            .map(|row| Width::from_db(WidthUid::from_uuid(row.width_uid), row.width))
            .collect()
    }

    async fn create(&self, width: &Width) -> Result<Width, MaterialError> {
        // Проверяем дубликаты
        if let Ok(Some(_)) = self.find_by_value(width.value()).await {
            return Err(MaterialError::DuplicateError {
                message: format!("Ширина {} уже существует", width.value()),
            });
        }

        let row = sqlx::query_as::<_, WidthDb>(
            "INSERT INTO materials.widths (width_uid, width) VALUES ($1, $2) RETURNING width_uid, width"
        )
        .bind(width.id().as_uuid())
        .bind(width.value())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка создания ширины: {}", e),
        })?;

        Width::from_db(WidthUid::from_uuid(row.width_uid), row.width)
    }

    async fn find_by_value(&self, value: f64) -> Result<Option<Width>, MaterialError> {
        let row = sqlx::query_as::<_, WidthDb>(
            "SELECT width_uid, width FROM materials.widths WHERE width = $1"
        )
        .bind(value)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка поиска ширины: {}", e),
        })?;

        match row {
            Some(row) => Ok(Some(Width::from_db(WidthUid::from_uuid(row.width_uid), row.width)?)),
            None => Ok(None),
        }
    }

    async fn exists(&self, id: WidthUid) -> Result<bool, MaterialError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM materials.widths WHERE width_uid = $1"
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