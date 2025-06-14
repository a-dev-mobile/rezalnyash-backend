use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use crate::features::materials::shared::errors::MaterialError;
use super::entity::{Thickness, ThicknessUid};

/// Модель БД для толщины
#[derive(Debug, FromRow)]
pub struct ThicknessDb {
    pub thickness_uid: Uuid,
    pub thickness: f64,
}

/// Трейт репозитория толщин
#[async_trait::async_trait]
pub trait ThicknessRepository: Send + Sync {
    async fn get_by_id(&self, id: ThicknessUid) -> Result<Thickness, MaterialError>;
    async fn get_all(&self) -> Result<Vec<Thickness>, MaterialError>;
    async fn create(&self, thickness: &Thickness) -> Result<Thickness, MaterialError>;
    async fn find_by_value(&self, value: f64) -> Result<Option<Thickness>, MaterialError>;
    async fn exists(&self, id: ThicknessUid) -> Result<bool, MaterialError>;
}

/// PostgreSQL реализация репозитория
pub struct PostgresThicknessRepository {
    pool: PgPool,
}

impl PostgresThicknessRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ThicknessRepository for PostgresThicknessRepository {
    async fn get_by_id(&self, id: ThicknessUid) -> Result<Thickness, MaterialError> {
        let row = sqlx::query_as::<_, ThicknessDb>(
            "SELECT thickness_uid, thickness FROM materials.thicknesses WHERE thickness_uid = $1"
        )
        .bind(id.as_uuid())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => MaterialError::NotFoundError {
                message: format!("Толщина с ID {} не найдена", id.as_uuid()),
            },
            _ => MaterialError::DatabaseError {
                message: format!("Ошибка БД: {}", e),
            },
        })?;

        Thickness::from_db(ThicknessUid::from_uuid(row.thickness_uid), row.thickness)
    }

    async fn get_all(&self) -> Result<Vec<Thickness>, MaterialError> {
        let rows = sqlx::query_as::<_, ThicknessDb>(
            "SELECT thickness_uid, thickness FROM materials.thicknesses ORDER BY thickness"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка получения толщин: {}", e),
        })?;

        rows.into_iter()
            .map(|row| Thickness::from_db(ThicknessUid::from_uuid(row.thickness_uid), row.thickness))
            .collect()
    }

    async fn create(&self, thickness: &Thickness) -> Result<Thickness, MaterialError> {
        // Проверяем дубликаты
        if let Ok(Some(_)) = self.find_by_value(thickness.value()).await {
            return Err(MaterialError::DuplicateError {
                message: format!("Толщина {} уже существует", thickness.value()),
            });
        }

        let row = sqlx::query_as::<_, ThicknessDb>(
            "INSERT INTO materials.thicknesses (thickness_uid, thickness) VALUES ($1, $2) RETURNING thickness_uid, thickness"
        )
        .bind(thickness.id().as_uuid())
        .bind(thickness.value())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка создания толщины: {}", e),
        })?;

        Thickness::from_db(ThicknessUid::from_uuid(row.thickness_uid), row.thickness)
    }

    async fn find_by_value(&self, value: f64) -> Result<Option<Thickness>, MaterialError> {
        let row = sqlx::query_as::<_, ThicknessDb>(
            "SELECT thickness_uid, thickness FROM materials.thicknesses WHERE thickness = $1"
        )
        .bind(value)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Ошибка поиска толщины: {}", e),
        })?;

        match row {
            Some(row) => Ok(Some(Thickness::from_db(ThicknessUid::from_uuid(row.thickness_uid), row.thickness)?)),
            None => Ok(None),
        }
    }

    async fn exists(&self, id: ThicknessUid) -> Result<bool, MaterialError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM materials.thicknesses WHERE thickness_uid = $1"
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