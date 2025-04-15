use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SheetDimension {
    pub width: u32,
    pub length: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetailDimension {
    pub id: u32,
    pub name: String,
    pub width: u32,
    pub length: u32,
    pub quantity: u32,
    pub angle: Option<u32>,  // угол поворота в градусах (0, 90, 180, 270)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EdgeType {
    pub edge_type: String,  // тип кромки
    pub thickness: f32,     // толщина кромки в мм
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SheetRequest {
    pub sheet: SheetDimension,
    pub material: MaterialInfo,
    pub details: Vec<DetailDimension>,
    pub layout: LayoutOptions,
    pub edges: Option<Vec<EdgeType>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaterialInfo {
    pub material_type: String,  // тип материала
    pub thickness: f32,         // толщина материала в мм
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LayoutOptions {
    pub method: String,         // "horizontal", "vertical", или "optimal"
    pub gap: u32,               // зазор между деталями в пикселях
    pub blade_width: u32,       // ширина реза пилы в мм
    pub margin: u32,            // отступ от края листа в мм
    pub starting_corner: String, // "top-left", "top-right", "bottom-left", "bottom-right"
}

// Структуры для ответа
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SheetResponse {
    pub sheet: SheetInfo,
    pub layout: LayoutInfo,
    pub details: Vec<DetailInfo>,
    pub unplaced_details: Vec<UnplacedDetailInfo>,  // New field for details that didn't fit
    pub statistics: Statistics,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SheetInfo {
    pub width: u32,
    pub length: u32,
    pub padding: u32,
    pub viewBox: ViewBox,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViewBox {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LayoutInfo {
    pub method: String,
    pub gap: u32,
    pub blade_width: u32,
    pub margin: u32,
    pub starting_corner: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetailInfo {
    pub id: u32,
    pub name: String,
    pub width: u32,
    pub length: u32,
    pub angle: u32,
    pub x: u32,
    pub y: u32,
    pub textPosition: TextPosition,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextPosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Statistics {
    pub sheet_area: u32,           // sheet area
    pub used_area: u32,            // used area
    pub waste_area: u32,           // waste area
    pub cut_length: u32,           // total cut length
    pub edge_length: u32,          // total edge length
    pub detail_count: u32,         // count of placed details
    pub unplaced_count: u32,       // count of unplaced details - new field
    pub efficiency: f32,           // cutting efficiency percentage
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnplacedDetailInfo {
    pub id: u32,
    pub name: String,
    pub width: u32,
    pub length: u32,
    pub angle: u32,
    pub quantity: u32,  // How many instances couldn't be placed
}