use crate::features::materials::{
    domain::{entities::Thickness, errors::MaterialError, traits::ThicknessBehavior, value_objects::ThicknessUid},
    models::database::ThicknessDb,
    repositories::{mappers::ThicknessMapper, traits::ThicknessRepository},
};
use sqlx::PgPool;

pub struct PostgresThicknessRepository {
    pool: PgPool,
}

impl PostgresThicknessRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ThicknessBehavior for PostgresThicknessRepository {
    async fn get_thickness(&self, id: &ThicknessUid) -> Result<Thickness, MaterialError> {
        let db_model = sqlx::query_as::<_, ThicknessDb>(
            "SELECT thickness_uid, thickness FROM materials.thicknesses WHERE thickness_uid = $1",
        )
        .bind(id.value())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => MaterialError::NotFoundError {
                message: format!("Толщина с ID {} не найдена", id.value()),
            },
            _ => MaterialError::DatabaseError {
                message: format!("Failed to get thickness: {}", e),
            },
        })?;

        ThicknessMapper::from_db(db_model)
    }

    async fn get_all_thicknesses(&self) -> Result<Vec<Thickness>, MaterialError> {
        let db_models = sqlx::query_as::<_, ThicknessDb>(
            "SELECT thickness_uid, thickness FROM materials.thicknesses ORDER BY thickness",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to get all thicknesses: {}", e),
        })?;

        ThicknessMapper::from_db_list(db_models)
    }

    async fn create_thickness(&self, thickness: Thickness) -> Result<Thickness, MaterialError> {
        if let Ok(existing) = self.find_by_value(thickness.thickness()).await {
            if existing.is_some() {
                return Err(MaterialError::DuplicateError {
                    message: format!("Толщина {} уже существует", thickness.thickness()),
                });
            }
        }

        let db_model = sqlx::query_as::<_, ThicknessDb>(
            "INSERT INTO materials.thicknesses (thickness_uid, thickness) VALUES ($1, $2) RETURNING thickness_uid, thickness"
        )
        .bind(thickness.id().value())
        .bind(thickness.thickness())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to create thickness: {}", e),
        })?;

        ThicknessMapper::from_db(db_model)
    }

    async fn exists(&self, id: &ThicknessUid) -> Result<bool, MaterialError> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM materials.thicknesses WHERE thickness_uid = $1")
            .bind(id.value())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| MaterialError::DatabaseError {
                message: format!("Failed to check thickness existence: {}", e),
            })?;

        Ok(count > 0)
    }

    async fn find_by_value(&self, value: f64) -> Result<Option<Thickness>, MaterialError> {
        let db_model = sqlx::query_as::<_, ThicknessDb>(
            "SELECT thickness_uid, thickness FROM materials.thicknesses WHERE thickness = $1",
        )
        .bind(value)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to find thickness by value: {}", e),
        })?;

        match db_model {
            Some(model) => Ok(Some(ThicknessMapper::from_db(model)?)),
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl ThicknessRepository for PostgresThicknessRepository {
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
