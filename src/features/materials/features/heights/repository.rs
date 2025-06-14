use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use crate::features::materials::shared::errors::MaterialError;
use super::entity::{Height, HeightUid};

/// Модель БД для высоты
#[derive(Debug, FromRow)]
pub struct HeightDb {
    pub height_uid: Uuid,
    pub height: f64,
}

/// Трейт репозитория высот
#[async_trait::async_trait]
pub trait HeightRepository: Send + Sync {
    async fn get_by_id(&self, id: HeightUid) -> Result<Height, MaterialError>;
    async fn get_all(&self) -> Result<Vec<Height>, MaterialError>;
    async fn create(&self, height: &Height) -> Result<Height, MaterialError>;
    async fn find_by_value(&self, value: f64) -> Result<Option<Height>, MaterialError>;
    async fn exists(&self, id: HeightUid) -> Result<bool, MaterialError>;
}

/// PostgreSQL реализация репозитория
pub struct PostgresHeightRepository {
    pool: PgPool,
}

impl PostgresHeightRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl HeightRepository for PostgresHeightRepository {
    async fn get_by_id(&self, id: HeightUid) -> Result<Height, MaterialError> {
        let row = sqlx::query_as::<_, HeightDb>(
            "SELECT height_uid, height FROM materials.heights WHERE height_uid = $1"
        )
        .bind(id.as_uuid())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => MaterialError::NotFoundError {
                message: format!("Высота с ID {} не найдена", id.as_uuid()),
            },
            _ => MaterialError::DatabaseError {
                message: format!("Ошибка БД: {}", e),
            },
        })?;

        Height::from_db(HeightUid::from_uuid(row.height_uid), row.height)
    }

    async fn get_all(&self) -> Result<Vec<Height>, MaterialError> {
        let rows = sqlx::query_as::<_, HeightDb>(
            "SELECT height_uid, height FROM materials.heights ORDER BY height"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка получения высот: {}", e),
        })?;

        rows.into_iter()
            .map(|row| Height::from_db(HeightUid::from_uuid(row.height_uid), row.height))
            .collect()
    }

    async fn create(&self, height: &Height) -> Result<Height, MaterialError> {
        // Проверяем дубликаты
        if let Ok(Some(_)) = self.find_by_value(height.value()).await {
            return Err(MaterialError::DuplicateError {
                message: format!("Высота {} уже существует", height.value()),
            });
        }

        let row = sqlx::query_as::<_, HeightDb>(
            "INSERT INTO materials.heights (height_uid, height) VALUES ($1, $2) RETURNING height_uid, height"
        )
        .bind(height.id().as_uuid())
        .bind(height.value())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка создания высоты: {}", e),
        })?;

        Height::from_db(HeightUid::from_uuid(row.height_uid), row.height)
    }

    async fn find_by_value(&self, value: f64) -> Result<Option<Height>, MaterialError> {
        let row = sqlx::query_as::<_, HeightDb>(
            "SELECT height_uid, height FROM materials.heights WHERE height = $1"
        )
        .bind(value)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка поиска высоты: {}", e),
        })?;

        match row {
            Some(row) => Ok(Some(Height::from_db(HeightUid::from_uuid(row.height_uid), row.height)?)),
            None => Ok(None),
        }
    }

    async fn exists(&self, id: HeightUid) -> Result<bool, MaterialError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM materials.heights WHERE height_uid = $1"
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