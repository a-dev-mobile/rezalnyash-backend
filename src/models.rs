use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SheetDimension {
    pub width: u32,
    pub length: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetailDimension {
    pub id: u32,
    pub width: u32,
    pub length: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SheetRequest {
    pub sheet: SheetDimension,
    pub details: Vec<DetailDimension>,
    pub layout: Option<LayoutOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LayoutOptions {
    pub orientation: String, // "horizontal" или "vertical"
    pub gap: u32,            // зазор между деталями в пикселях
}
