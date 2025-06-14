use sqlx::PgPool;
use crate::features::materials::{
    domain::{entities::Height, errors::MaterialError, value_objects::HeightUid, traits::HeightBehavior},
    models::database::HeightDb,
    repositories::{mappers::HeightMapper, traits::HeightRepository},
};

pub struct PostgresHeightRepository {
    pool: PgPool,
}

impl PostgresHeightRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl HeightBehavior for PostgresHeightRepository {
    async fn get_height(&self, id: &HeightUid) -> Result<Height, MaterialError> {
        let db_model = sqlx::query_as::<_, HeightDb>(
            "SELECT height_uid, height FROM materials.heights WHERE height_uid = $1"
        )
        .bind(id.value())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => MaterialError::NotFoundError {
                message: format!("Высота с ID {} не найдена", id.value()),
            },
            _ => MaterialError::DatabaseError {
                message: format!("Failed to get height: {}", e),
            },
        })?;

        HeightMapper::from_db(db_model)
    }

    async fn get_all_heights(&self) -> Result<Vec<Height>, MaterialError> {
        let db_models = sqlx::query_as::<_, HeightDb>(
            "SELECT height_uid, height FROM materials.heights ORDER BY height"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to get all heights: {}", e),
        })?;

        HeightMapper::from_db_list(db_models)
    }

    async fn create_height(&self, height: Height) -> Result<Height, MaterialError> {
        if let Ok(existing) = self.find_by_value(height.height()).await {
            if existing.is_some() {
                return Err(MaterialError::DuplicateError {
                    message: format!("Высота {} уже существует", height.height()),
                });
            }
        }

        let db_model = sqlx::query_as::<_, HeightDb>(
            "INSERT INTO materials.heights (height_uid, height) VALUES ($1, $2) RETURNING height_uid, height"
        )
        .bind(height.id().value())
        .bind(height.height())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to create height: {}", e),
        })?;

        HeightMapper::from_db(db_model)
    }

    async fn exists(&self, id: &HeightUid) -> Result<bool, MaterialError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM materials.heights WHERE height_uid = $1"
        )
        .bind(id.value())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to check height existence: {}", e),
        })?;

        Ok(count > 0)
    }

    async fn find_by_value(&self, value: f64) -> Result<Option<Height>, MaterialError> {
        let db_model = sqlx::query_as::<_, HeightDb>(
            "SELECT height_uid, height FROM materials.heights WHERE height = $1"
        )
        .bind(value)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to find height by value: {}", e),
        })?;

        match db_model {
            Some(model) => Ok(Some(HeightMapper::from_db(model)?)),
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl HeightRepository for PostgresHeightRepository {
    async fn health_check(&self) -> Result<(), MaterialError> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| MaterialError::DatabaseError {
                message: format!("Health check failed: {}", e),
            })?;
        Ok(())
    }
}