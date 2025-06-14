
use crate::features::materials::{
    services::dto::{CreateMaterialTypeDto, CreateMaterialNameDto},
    models::api::v1::{
        requests::{CreateMaterialTypeRequest, CreateMaterialNameRequest},
    },
};

pub struct ApiV1Converter;

impl ApiV1Converter {
    /// Конвертация CreateMaterialTypeRequest в CreateMaterialTypeDto
    pub fn create_material_type_request_to_dto(request: CreateMaterialTypeRequest) -> CreateMaterialTypeDto {
        CreateMaterialTypeDto::new(request.name_ru, request.name_en)
    }

    /// Конвертация CreateMaterialNameRequest в CreateMaterialNameDto
    pub fn create_material_name_request_to_dto(request: CreateMaterialNameRequest) -> CreateMaterialNameDto {
        CreateMaterialNameDto::new(request.name_ru, request.name_en)
    }
}