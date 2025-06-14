
use sqlx::PgPool;
use uuid::Uuid;
use crate::features::materials::{
    domain::{
        entities::MaterialName,
        errors::MaterialError,
        value_objects::MaterialNameUid,
        traits::MaterialNameBehavior,
    },
    models::database::MaterialNameDb,
    repositories::{
        mappers::MaterialNameMapper,
        traits::MaterialNameRepository,
    },
};

pub struct PostgresMaterialNameRepository {
    pool: PgPool,
}

impl PostgresMaterialNameRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl MaterialNameBehavior for PostgresMaterialNameRepository {
    async fn get_material_name(&self, id: &MaterialNameUid) -> Result<MaterialName, MaterialError> {
        let db_model = sqlx::query_as::<_, MaterialNameDb>(
            "SELECT material_name_uid, name_ru, name_en FROM materials.material_names WHERE material_name_uid = $1"
        )
        .bind(id.value())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => MaterialError::NotFoundError {
                message: format!("Название материала с ID {} не найдено", id.value()),
            },
            _ => MaterialError::DatabaseError {
                message: format!("Failed to get material name: {}", e),
            },
        })?;

        MaterialNameMapper::from_db(db_model)
    }

    async fn get_all_material_names(&self) -> Result<Vec<MaterialName>, MaterialError> {
        let db_models = sqlx::query_as::<_, MaterialNameDb>(
            "SELECT material_name_uid, name_ru, name_en FROM materials.material_names ORDER BY name_ru"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to get all material names: {}", e),
        })?;

        MaterialNameMapper::from_db_list(db_models)
    }

    async fn create_material_name(&self, material_name: MaterialName) -> Result<MaterialName, MaterialError> {
        // Проверяем, не существует ли уже такое название
        if let Ok(existing) = self.find_by_name(material_name.name_ru(), material_name.name_en()).await {
            if existing.is_some() {
                return Err(MaterialError::DuplicateError {
                    message: format!(
                        "Название материала '{}' / '{}' уже существует",
                        material_name.name_ru(),
                        material_name.name_en()
                    ),
                });
            }
        }

        let db_model = sqlx::query_as::<_, MaterialNameDb>(
            "INSERT INTO materials.material_names (material_name_uid, name_ru, name_en) VALUES ($1, $2, $3) RETURNING material_name_uid, name_ru, name_en"
        )
        .bind(material_name.id().value())
        .bind(material_name.name_ru())
        .bind(material_name.name_en())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to create material name: {}", e),
        })?;

        MaterialNameMapper::from_db(db_model)
    }

    async fn exists(&self, id: &MaterialNameUid) -> Result<bool, MaterialError> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM materials.material_names WHERE material_name_uid = $1"
        )
        .bind(id.value())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to check material name existence: {}", e),
        })?;

        Ok(count > 0)
    }

    async fn find_by_name(&self, name_ru: &str, name_en: &str) -> Result<Option<MaterialName>, MaterialError> {
        let db_model = sqlx::query_as::<_, MaterialNameDb>(
            "SELECT material_name_uid, name_ru, name_en FROM materials.material_names WHERE name_ru = $1 AND name_en = $2"
        )
        .bind(name_ru)
        .bind(name_en)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to find material name by name: {}", e),
        })?;

        match db_model {
            Some(model) => Ok(Some(MaterialNameMapper::from_db(model)?)),
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl MaterialNameRepository for PostgresMaterialNameRepository {
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