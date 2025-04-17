// mod.rs - Определение модуля и экспорт функций
//
// Экспортирует основные функции и структуры из модуля svg_generator.

mod layout_horizontal;
mod layout_vertical;
mod layout_optimal;
mod svg_builder;
mod svg_generator;

pub use layout_horizontal::horizontal_layout;
pub use layout_vertical::vertical_layout;
pub use layout_optimal::optimal_layout;
pub use svg_builder::SvgBuilder;
pub use svg_generator::generate_sheet_svg;

use std::collections::HashMap;
use log::info;