use crate::features::materials::{
    domain::{
        entities::MaterialType, errors::MaterialError, traits::MaterialTypeBehavior, value_objects::MaterialTypeId,
    },
    models::database::MaterialTypeDb,
    repositories::{mappers::MaterialTypeMapper, traits::MaterialTypeRepository},
};
use sqlx::PgPool;

pub struct PostgresMaterialTypeRepository {
    pool: PgPool,
}

impl PostgresMaterialTypeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl MaterialTypeBehavior for PostgresMaterialTypeRepository {
    async fn get_material_type(&self, id: &MaterialTypeId) -> Result<MaterialType, MaterialError> {
        let db_model = sqlx::query_as::<_, MaterialTypeDb>(
            "SELECT material_type_id, name_ru, name_en FROM materials.material_types WHERE material_type_id = $1",
        )
        .bind(id.value())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => MaterialError::NotFoundError {
                message: format!("Тип материала с ID {} не найден", id.value()),
            },
            _ => MaterialError::DatabaseError {
                message: format!("Failed to get material type: {}", e),
            },
        })?;

        MaterialTypeMapper::from_db(db_model)
    }

    async fn get_all_material_types(&self) -> Result<Vec<MaterialType>, MaterialError> {
        let db_models = sqlx::query_as::<_, MaterialTypeDb>(
            "SELECT material_type_id, name_ru, name_en FROM materials.material_types ORDER BY material_type_id",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| MaterialError::DatabaseError {
            message: format!("Failed to get all material types: {}", e),
        })?;

        MaterialTypeMapper::from_db_list(db_models)
    }

    async fn create_material_type(&self, material_type: MaterialType) -> Result<MaterialType, MaterialError> {
        // Проверяем, не существует ли уже такой тип
        if let Ok(existing) = self
            .find_by_name(material_type.name_ru(), material_type.name_en())
            .await
        {
            if existing.is_some() {
                return Err(MaterialError::DuplicateError {
                    message: format!(
                        "Тип материала с названиями '{}' / '{}' уже существует",
                        material_type.name_ru(),
                        material_type.name_en()
                    ),
                });
            }
        }

        let db_model = sqlx::query_as::<_, MaterialTypeDb>("INSERT INTO materials.material_types (name_ru, name_en) VALUES ($1, $2) RETURNING material_type_id, name_ru, name_en")
            .bind(material_type.name_ru())
            .bind(material_type.name_en())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| MaterialError::DatabaseError {
                message: format!("Failed to create material type: {}", e),
            })?;

        MaterialTypeMapper::from_db(db_model)
    }

    async fn exists(&self, id: &MaterialTypeId) -> Result<bool, MaterialError> {
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM materials.material_types WHERE material_type_id = $1")
                .bind(id.value())
                .fetch_one(&self.pool)
                .await
                .map_err(|e| MaterialError::DatabaseError {
                    message: format!("Failed to check material type existence: {}", e),
                })?;

        Ok(count > 0)
    }

    async fn find_by_name(&self, name_ru: &str, name_en: &str) -> Result<Option<MaterialType>, MaterialError> {
        let db_model = sqlx::query_as::<_, MaterialTypeDb>("SELECT material_type_id, name_ru, name_en FROM materials.material_types WHERE name_ru = $1 AND name_en = $2")
            .bind(name_ru)
            .bind(name_en)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| MaterialError::DatabaseError {
                message: format!("Failed to find material type by name: {}", e),
            })?;

        match db_model {
            Some(model) => Ok(Some(MaterialTypeMapper::from_db(model)?)),
            None => Ok(None),
        }
    }
}

#[async_trait::async_trait]
impl MaterialTypeRepository for PostgresMaterialTypeRepository {
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