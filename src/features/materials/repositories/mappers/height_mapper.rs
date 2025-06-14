use uuid::Uuid;
use crate::features::materials::{
    domain::{entities::Height, errors::MaterialError, value_objects::HeightUid},
    models::database::HeightDb,
};

pub struct HeightMapper;

impl HeightMapper {
    pub fn to_db_insert(domain: &Height) -> HeightDb {
        HeightDb::new(Uuid::new_v4(), domain.height())
    }

    pub fn from_db(db: HeightDb) -> Result<Height, MaterialError> {
        let id = HeightUid::new(db.height_uid);
        Height::new(id, db.height)
    }

    pub fn to_db_update(domain: &Height) -> HeightDb {
        HeightDb::new(domain.id().value(), domain.height())
    }

    pub fn from_db_list(db_list: Vec<HeightDb>) -> Result<Vec<Height>, MaterialError> {
        db_list.into_iter().map(Self::from_db).collect::<Result<Vec<_>, _>>()
    }
}