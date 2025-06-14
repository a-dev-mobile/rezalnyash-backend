
use uuid::Uuid;

use crate::features::materials::{
    domain::{
        entities::MaterialName,
        errors::MaterialError,
        value_objects::MaterialNameUid,
    },
    models::database::MaterialNameDb,
};

pub struct MaterialNameMapper;

impl MaterialNameMapper {
    /// Преобразование из доменной модели в модель БД (для вставки)
    pub fn to_db_insert(domain: &MaterialName) -> MaterialNameDb {
        MaterialNameDb::new(
            Uuid::new_v4(),
            domain.name_ru().to_string(),
            domain.name_en().to_string(),
        )
    }

    /// Преобразование из модели БД в доменную модель
    pub fn from_db(db: MaterialNameDb) -> Result<MaterialName, MaterialError> {
        let id = MaterialNameUid::new(db.material_name_uid);
        MaterialName::new(id, db.name_ru, db.name_en)
    }

    /// Преобразование из доменной модели в модель БД (для обновления)
    pub fn to_db_update(domain: &MaterialName) -> MaterialNameDb {
        MaterialNameDb::new(
            domain.id().value(),
            domain.name_ru().to_string(),
            domain.name_en().to_string(),
        )
    }

    /// Преобразование списка из БД в доменные модели
    pub fn from_db_list(db_list: Vec<MaterialNameDb>) -> Result<Vec<MaterialName>, MaterialError> {
        db_list
            .into_iter()
            .map(Self::from_db)
            .collect::<Result<Vec<_>, _>>()
    }
}