// svg_generator.rs - Основной модуль для генерации SVG с раскроем листовых материалов
//
// Экспортирует основные функции для генерации SVG-схем раскроя
// и координирует взаимодействие между разными алгоритмами размещения

use std::collections::HashMap;
use log::info;
use crate::models::{margins::Margins, other::{DetailDimension, DetailInfo, LayoutOptions, SheetDimension, SheetRequest, TextPosition}};

use super::{horizontal_layout, optimal_layout, vertical_layout, SvgBuilder};

/// Генерирует SVG с раскроем листа и размещенными деталями
pub fn generate_sheet_svg(request: &SheetRequest) -> String {
    let sheet = &request.sheet;
    let details = &request.details;
    let layout = &request.layout;

    // Определяем максимальный отступ для SVG
    let max_margin = std::cmp::max(
        std::cmp::max(layout.margin.top, layout.margin.bottom),
        std::cmp::max(layout.margin.left, layout.margin.right)
    );
    
    // Определяем размеры SVG и viewBox
    let width = sheet.width + layout.margin.left + layout.margin.right;
    let height = sheet.length + layout.margin.top + layout.margin.bottom;
    
    // Создаем структуры для деталей
    let mut details_data = Vec::new();
    let mut unplaced_details_map = HashMap::new();
    
    // Определяем начальную позицию в зависимости от выбранного угла
    let (mut current_x, mut current_y) = match layout.starting_corner.as_str() {
        "top-left" => (layout.margin.left, layout.margin.top),
        "top-right" => (sheet.width + layout.margin.left, layout.margin.top),
        "bottom-left" => (layout.margin.left, sheet.length + layout.margin.top),
        "bottom-right" => (sheet.width + layout.margin.left, sheet.length + layout.margin.top),
        _ => (layout.margin.left, layout.margin.top), // По умолчанию верхний левый угол
    };

    // Направление размещения зависит от выбранного угла
    let (x_step, y_step) = match layout.starting_corner.as_str() {
        "top-left" => (1, 1),
        "top-right" => (-1, 1),
        "bottom-left" => (1, -1),
        "bottom-right" => (-1, -1),
        _ => (1, 1), // По умолчанию верхний левый угол
    };

    // Выбираем метод размещения в зависимости от настроек
    match layout.method.as_str() {
        "horizontal" => horizontal_layout(
            details, sheet, layout, &layout.margin, x_step, y_step, 
            &mut current_x, &mut current_y, &mut details_data, &mut unplaced_details_map
        ),
        "vertical" => vertical_layout(
            details, sheet, layout, &layout.margin, x_step, y_step, 
            &mut current_x, &mut current_y, &mut details_data, &mut unplaced_details_map
        ),
        "optimal" => optimal_layout(
            details, sheet, layout, &layout.margin, &mut details_data, &mut unplaced_details_map
        ),
        _ => horizontal_layout(
            details, sheet, layout, &layout.margin, x_step, y_step, 
            &mut current_x, &mut current_y, &mut details_data, &mut unplaced_details_map
        ),
    }

    // Создаем и возвращаем SVG
    let mut builder = SvgBuilder::new(width, height);
    builder.add_sheet(layout.margin.left, layout.margin.top, sheet.width, sheet.length);
    builder.add_margin_indicators(0, 0, width, height);
    
    // Добавляем размещенные детали
    for detail in &details_data {
        builder.add_detail(detail);
    }
    
    builder.build()
}