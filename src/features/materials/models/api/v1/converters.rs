use crate::features::materials::{
    models::api::v1::{
        requests::{CreateMaterialNameRequest, CreateMaterialTypeRequest},
        CreateHeightRequest, CreateThicknessRequest, CreateWidthRequest,
    },
    services::dto::{CreateHeightDto, CreateMaterialNameDto, CreateMaterialTypeDto, CreateThicknessDto, CreateWidthDto},
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
    /// Конвертация CreateWidthRequest в CreateWidthDto
    pub fn create_width_request_to_dto(request: CreateWidthRequest) -> CreateWidthDto {
        CreateWidthDto::new(request.width)
    }

    /// Конвертация CreateHeightRequest в CreateHeightDto
    pub fn create_height_request_to_dto(request: CreateHeightRequest) -> CreateHeightDto {
        CreateHeightDto::new(request.height)
    }

    /// Конвертация CreateThicknessRequest в CreateThicknessDto
    pub fn create_thickness_request_to_dto(request: CreateThicknessRequest) -> CreateThicknessDto {
        CreateThicknessDto::new(request.thickness)
    }
}
