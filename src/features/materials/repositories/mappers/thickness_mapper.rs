use uuid::Uuid;
use crate::features::materials::{
    domain::{entities::Thickness, errors::MaterialError, value_objects::ThicknessUid},
    models::database::ThicknessDb,
};

pub struct ThicknessMapper;

impl ThicknessMapper {
    pub fn to_db_insert(domain: &Thickness) -> ThicknessDb {
        ThicknessDb::new(Uuid::new_v4(), domain.thickness())
    }

    pub fn from_db(db: ThicknessDb) -> Result<Thickness, MaterialError> {
        let id = ThicknessUid::new(db.thickness_uid);
        Thickness::new(id, db.thickness)
    }

    pub fn to_db_update(domain: &Thickness) -> ThicknessDb {
        ThicknessDb::new(domain.id().value(), domain.thickness())
    }

    pub fn from_db_list(db_list: Vec<ThicknessDb>) -> Result<Vec<Thickness>, MaterialError> {
        db_list.into_iter().map(Self::from_db).collect::<Result<Vec<_>, _>>()
    }
}