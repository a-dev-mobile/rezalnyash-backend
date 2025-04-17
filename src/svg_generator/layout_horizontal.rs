// layout_horizontal.rs - Модуль горизонтального размещения деталей
//
// Реализует алгоритм горизонтального размещения деталей на листе материала.
// Оптимизирован для минимизации отходов при горизонтальных резах.

use std::collections::HashMap;

use crate::models::{margins::Margins, other::{DetailDimension, DetailInfo, LayoutOptions, SheetDimension, TextPosition, UnplacedDetailInfo}};

/// Функция для горизонтального размещения деталей на листе
/// 
/// # Аргументы
/// * `details` - Вектор деталей для размещения
/// * `sheet` - Размеры листа
/// * `layout` - Параметры размещения
/// * `margins` - Отступы от краев листа
/// * `x_step` - Направление по оси X (1 = вправо, -1 = влево)
/// * `y_step` - Направление по оси Y (1 = вниз, -1 = вверх)
/// * `current_x` - Начальная позиция по X
/// * `current_y` - Начальная позиция по Y
/// * `details_data` - Вектор для сохранения информации о размещенных деталях
/// * `unplaced_details_map` - Карта для отслеживания непомещенных деталей
pub fn horizontal_layout(
    details: &[DetailDimension],
    sheet: &SheetDimension,
    layout: &LayoutOptions,
    margins: &Margins,
    x_step: i32,
    y_step: i32,
    current_x: &mut u32,
    current_y: &mut u32,
    details_data: &mut Vec<DetailInfo>,
    unplaced_details_map: &mut HashMap<u32, UnplacedDetailInfo>,
) {
    let gap = layout.gap;
    let blade_width = layout.blade_width;

    // Сортируем детали по площади (от большей к меньшей)
    // Это помогает более эффективно использовать пространство листа
    let mut sorted_details = details.to_vec();
    sorted_details.sort_by(|a, b| {
        let area_a = a.width * a.height;
        let area_b = b.width * b.height;
        area_b.cmp(&area_a)
    });

    // Обрабатываем каждую деталь
    for detail in &sorted_details {
        let detail_id = detail.id;
        let mut placed_count = 0;
        
        // Пытаемся разместить каждый экземпляр детали
        for _ in 0..detail.quantity {
            // Определяем, нужно ли повернуть деталь для оптимизации раскроя
            // Для горизонтального размещения предпочтительнее детали с меньшей шириной
            let (effective_width, effective_length, angle) = match detail.angle {
                Some(ang) => (detail.width, detail.height, ang),
                None => {
                    // Автоматический поворот для оптимизации при горизонтальном размещении
                    if detail.width <= detail.height {
                        (detail.width, detail.height, 0)
                    } else {
                        (detail.height, detail.width, 90)
                    }
                }
            };
            
            // Проверяем, поместится ли деталь по оставшейся ширине
            let next_x = if x_step > 0 {
                *current_x as i32 + effective_width as i32 + gap as i32 + blade_width as i32
            } else {
                *current_x as i32 - effective_width as i32 - gap as i32 - blade_width as i32
            };
            
            // Вычисляем границы листа с учетом отступов
            let min_x = margins.left as i32;
            let max_x = (sheet.width + margins.left) as i32;
            
            // Если деталь не помещается по ширине, переходим на новую строку
            if (x_step > 0 && next_x > max_x) ||
               (x_step < 0 && next_x < min_x) {
                // Переходим на новую строку
                *current_x = if x_step > 0 { margins.left } else { sheet.width + margins.left };
                *current_y = if y_step > 0 {
                    *current_y + gap + blade_width + (&sorted_details).iter().map(|d| d.height).max().unwrap_or(0)
                } else {
                    let max_length = (&sorted_details).iter().map(|d| d.height).max().unwrap_or(0) as i32;
                    (*current_y as i32 - gap as i32 - blade_width as i32 - max_length).max(margins.top as i32) as u32
                };
            }

            // Проверяем границы листа по оси Y с учетом отступов
            let min_y = margins.top as i32;
            let max_y = (sheet.height + margins.top) as i32;
            
            // Проверяем, поместится ли деталь по высоте
            if (y_step > 0 && (*current_y + effective_length) > (sheet.height + margins.top)) ||
               (y_step < 0 && (*current_y as i32 - effective_length as i32) < min_y) {
                // Если деталь не помещается по высоте, она не может быть размещена
                continue;
            }

            // Вычисляем координаты размещения детали
            let x_pos = if x_step < 0 { 
                (*current_x as i32 - effective_width as i32).max(min_x) as u32 
            } else { 
                *current_x 
            };
            
            let y_pos = if y_step < 0 { 
                (*current_y as i32 - effective_length as i32).max(min_y) as u32 
            } else { 
                *current_y 
            };
            
            // Добавляем информацию о размещенной детали
            details_data.push(DetailInfo {
                id: detail.id,
                name: detail.name.clone(),
                width: effective_width,
                height: effective_length,
                angle,
                x: x_pos,
                y: y_pos,
                textPosition: TextPosition {
                    x: x_pos + effective_width / 2,
                    y: y_pos + effective_length / 2
                }
            });
            
            placed_count += 1;

            // Перемещаем указатель для следующей детали
            if x_step > 0 {
                *current_x += effective_width + gap + blade_width;
            } else {
                let new_x = (*current_x as i32 - effective_width as i32 - gap as i32 - blade_width as i32).max(min_x);
                *current_x = new_x as u32;
            }
        }
        
        // Обновляем карту непомещенных деталей
        if let Some(unplaced_detail) = unplaced_details_map.get_mut(&detail_id) {
            unplaced_detail.quantity = detail.quantity - placed_count;
        }
    }
}