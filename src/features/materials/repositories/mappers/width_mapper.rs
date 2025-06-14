use uuid::Uuid;
use crate::features::materials::{
    domain::{entities::Width, errors::MaterialError, value_objects::WidthUid},
    models::database::WidthDb,
};

pub struct WidthMapper;

impl WidthMapper {
    pub fn to_db_insert(domain: &Width) -> WidthDb {
        WidthDb::new(Uuid::new_v4(), domain.width())
    }

    pub fn from_db(db: WidthDb) -> Result<Width, MaterialError> {
        let id = WidthUid::new(db.width_uid);
        Width::new(id, db.width)
    }

    pub fn to_db_update(domain: &Width) -> WidthDb {
        WidthDb::new(domain.id().value(), domain.width())
    }

    pub fn from_db_list(db_list: Vec<WidthDb>) -> Result<Vec<Width>, MaterialError> {
        db_list.into_iter().map(Self::from_db).collect::<Result<Vec<_>, _>>()
    }
}