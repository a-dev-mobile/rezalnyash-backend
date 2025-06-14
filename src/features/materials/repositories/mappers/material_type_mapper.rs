
use crate::features::materials::{
    domain::{
        entities::MaterialType,
        errors::MaterialError,
        value_objects::MaterialTypeId,
    },
    models::database::MaterialTypeDb,
};

pub struct MaterialTypeMapper;

impl MaterialTypeMapper {
    /// Преобразование из доменной модели в модель БД (для вставки)
    pub fn to_db_insert(domain: &MaterialType) -> MaterialTypeDb {
        MaterialTypeDb::new(
            0, // ID будет назначен БД
            domain.name_ru().to_string(),
            domain.name_en().to_string(),
        )
    }

    /// Преобразование из модели БД в доменную модель
    pub fn from_db(db: MaterialTypeDb) -> Result<MaterialType, MaterialError> {
        let id = MaterialTypeId::new(db.material_type_id)?;
        MaterialType::new(id, db.name_ru, db.name_en)
    }

    /// Преобразование из доменной модели в модель БД (для обновления)
    pub fn to_db_update(domain: &MaterialType) -> MaterialTypeDb {
        MaterialTypeDb::new(
            domain.id().value(),
            domain.name_ru().to_string(),
            domain.name_en().to_string(),
        )
    }

    /// Преобразование списка из БД в доменные модели
    pub fn from_db_list(db_list: Vec<MaterialTypeDb>) -> Result<Vec<MaterialType>, MaterialError> {
        db_list
            .into_iter()
            .map(Self::from_db)
            .collect::<Result<Vec<_>, _>>()
    }
}
