use sqlx::PgPool;
use crate::features::materials::{
    domain::{entities::Width, errors::MaterialError, value_objects::WidthUid, traits::WidthBehavior},
    models::database::WidthDb,
    repositories::{mappers::WidthMapper, traits::WidthRepository},
};

pub struct PostgresWidthRepository {
    pool: PgPool,
}

impl PostgresWidthRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl WidthBehavior for PostgresWidthRepository {
    async fn get_width(&self, id: &WidthUid) -> Result<Width, MaterialError> {
        let db_model = sqlx::query_as::<_, WidthDb>(
            "SELECT width_uid, width FROM materials.widths WHERE width_uid = $1"
        )
        .bind(id.value())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => MaterialError::NotFoundError {
                message: format!("Ширина с ID {} не найдена", id.value()),
            },
            _ => MaterialError::DatabaseError {
                message: format!("Failed to get width: {}", e),
            },
        })?;

        WidthMapper::from_db(db_model)
    }

    async fn get_all_widths(&self) -> Result<Vec<Width>, MaterialError> {
        let db_models = sqlx::query_as::<_, WidthDb>(
            "SELECT width_uid, width FROM materials.widths ORDER BY width"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to get all widths: {}", e),
        })?;

        WidthMapper::from_db_list(db_models)
    }

    async fn create_width(&self, width: Width) -> Result<Width, MaterialError> {
        if let Ok(existing) = self.find_by_value(width.width()).await {
            if existing.is_some() {
                return Err(MaterialError::DuplicateError {
                    message: format!("Ширина {} уже существует", width.width()),
                });
            }
        }

        let db_model = sqlx::query_as::<_, WidthDb>(
            "INSERT INTO materials.widths (width_uid, width) VALUES ($1, $2) RETURNING width_uid, width"
        )
        .bind(width.id().value())
        .bind(width.width())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to create width: {}", e),
        })?;

        WidthMapper::from_db(db_model)
    }

    async fn exists(&self, id: &WidthUid) -> Result<bool, MaterialError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM materials.widths WHERE width_uid = $1"
        )
        .bind(id.value())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to check width existence: {}", e),
        })?;

        Ok(count > 0)
    }

    async fn find_by_value(&self, value: f64) -> Result<Option<Width>, MaterialError> {
        let db_model = sqlx::query_as::<_, WidthDb>(
            "SELECT width_uid, width FROM materials.widths WHERE width = $1"
        )
        .bind(value)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to find width by value: {}", e),
        })?;

        match db_model {
            Some(model) => Ok(Some(WidthMapper::from_db(model)?)),
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl WidthRepository for PostgresWidthRepository {
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