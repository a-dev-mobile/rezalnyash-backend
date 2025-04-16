use crate::models::{DetailDimension, DetailInfo, LayoutOptions, SheetDimension, SheetRequest, Statistics, TextPosition, UnplacedDetailInfo};
use std::fmt::Write;
use std::collections::HashMap;

/// Генерирует SVG с раскроем листа и размещенными деталями
pub fn generate_sheet_svg(request: &SheetRequest) -> String {
    let sheet = &request.sheet;
    let details = &request.details;
    let layout = &request.layout;

    // Используем отступ из параметров запроса
    let padding = layout.margin;
    
    // Определяем размеры SVG и viewBox
    let width = sheet.width + 2 * padding;
    let height = sheet.length + 2 * padding;
    
    // Создаем данные для деталей и неразмещенных деталей
    let mut details_data = Vec::new();
    let mut unplaced_details_map: HashMap<u32, UnplacedDetailInfo> = HashMap::new();
    
    // Инициализируем unplaced_details_map всеми деталями из запроса
    for detail in details {
        let detail_id = detail.id;
        unplaced_details_map.insert(detail_id, UnplacedDetailInfo {
            id: detail.id,
            name: detail.name.clone(),
            width: detail.width,
            length: detail.length,
            angle: detail.angle.unwrap_or(0),
            quantity: detail.quantity,
        });
    }
    
    // Определяем начальную позицию в зависимости от starting_corner
    let (mut current_x, mut current_y) = match layout.starting_corner.as_str() {
        "top-left" => (padding, padding),
        "top-right" => (sheet.width, padding),
        "bottom-left" => (padding, sheet.length),
        "bottom-right" => (sheet.width, sheet.length),
        _ => (padding, padding), // По умолчанию top-left
    };

    // Направление движения зависит от starting_corner
    let (x_step, y_step) = match layout.starting_corner.as_str() {
        "top-left" => (1, 1),
        "top-right" => (-1, 1),
        "bottom-left" => (1, -1),
        "bottom-right" => (-1, -1),
        _ => (1, 1), // По умолчанию top-left
    };

    // Вычисляем позиции в зависимости от метода макета
    match layout.method.as_str() {
        "horizontal" => horizontal_layout(
            details, sheet, layout, padding, x_step, y_step, 
            &mut current_x, &mut current_y, &mut details_data, &mut unplaced_details_map
        ),
        "vertical" => vertical_layout(
            details, sheet, layout, padding, x_step, y_step, 
            &mut current_x, &mut current_y, &mut details_data, &mut unplaced_details_map
        ),
        "optimal" => optimal_layout_stub(
            details, sheet, layout, padding, &mut details_data, &mut unplaced_details_map
        ),
        _ => horizontal_layout(
            details, sheet, layout, padding, x_step, y_step, 
            &mut current_x, &mut current_y, &mut details_data, &mut unplaced_details_map
        ),
    }

    // Преобразуем unplaced_details_map в вектор
    let unplaced_details: Vec<UnplacedDetailInfo> = unplaced_details_map
        .into_iter()
        .filter(|(_, detail)| detail.quantity > 0)  // Включаем только детали с quantity > 0
        .map(|(_, detail)| detail)
        .collect();

    // Рассчитываем статистику
    let statistics = calculate_statistics(sheet, &details_data, &unplaced_details);
    
    // Создаем строку для SVG
    let mut svg = String::new();
    
    // Объявление XML и элемента SVG с размерами
    writeln!(
        &mut svg,
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="{}" height="{}" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
        width, height, width, height
    ).unwrap();
    
    // Определение стилей
    writeln!(
        &mut svg,
        r#"<style>
    /* Стиль для листа материала */
    .sheet {{ fill: #f0f0f0; stroke: #000000; stroke-width: 2; }}
    /* Стиль для размещенных деталей */
    .detail {{ fill: #a0d0ff; stroke: #000000; stroke-width: 1; }}
    /* Стиль для неразмещенных деталей */
    .unplaced {{ fill: #ffa0a0; stroke: #000000; stroke-width: 1; }}
    /* Стиль для текста идентификаторов деталей */
    .text {{ font-family: Arial, sans-serif; font-size: 14px; text-anchor: middle; }}
    /* Стиль для текста статистики */
    .stats {{ font-family: Arial, sans-serif; font-size: 12px; }}
</style>"#
    ).unwrap();
    
    // Рисуем лист материала
    writeln!(
        &mut svg,
        r#"<rect class="sheet" x="{}" y="{}" width="{}" height="{}" />"#,
        padding, padding, sheet.width, sheet.length
    ).unwrap();
    
    // Добавляем размещенные детали
    for detail in &details_data {
        // Определяем цвет детали в зависимости от её ID
        let hue = (detail.id * 60) % 360;
        
        // Рисуем прямоугольник детали
        writeln!(
            &mut svg,
            r#"<rect class="detail" x="{}" y="{}" width="{}" height="{}" style="fill: hsl({}, 70%, 80%);" />"#,
            detail.x, detail.y, detail.width, detail.length, hue
        ).unwrap();
        
        // Добавляем текст детали
        writeln!(
            &mut svg,
            r#"<text class="text" x="{}" y="{}">{} ({})</text>"#,
            detail.textPosition.x, detail.textPosition.y, detail.name, detail.id
        ).unwrap();
    }
    
    // Добавляем статистику
    writeln!(
        &mut svg,
        r#"<text class="stats" x="{}" y="{}">Эффективность: {:.1}%</text>"#,
        padding, height - 60, statistics.efficiency
    ).unwrap();
    
    writeln!(
        &mut svg,
        r#"<text class="stats" x="{}" y="{}">Использовано: {} из {} (отходы: {})</text>"#,
        padding, height - 45, 
        statistics.used_area, 
        statistics.sheet_area,
        statistics.waste_area
    ).unwrap();
    
    writeln!(
        &mut svg,
        r#"<text class="stats" x="{}" y="{}">Размещено деталей: {}, не размещено: {}</text>"#,
        padding, height - 30,
        statistics.detail_count,
        statistics.unplaced_count
    ).unwrap();
    
    writeln!(
        &mut svg,
        r#"<text class="stats" x="{}" y="{}">Метод раскроя: {}, зазор: {}, ширина реза: {}</text>"#,
        padding, height - 15,
        layout.method,
        layout.gap,
        layout.blade_width
    ).unwrap();
    
    // Закрываем SVG
    writeln!(&mut svg, "</svg>").unwrap();
    
    svg
}

/// Генерирует SVG с неразмещенными деталями
pub fn generate_unplaced_svg(request: &SheetRequest) -> String {
    let details = &request.details;
    let layout = &request.layout;
    let padding = 20; // Отступ для SVG с неразмещенными деталями
    
    // Создаем данные для деталей и неразмещенных деталей
    let mut details_data = Vec::new();
    let mut unplaced_details_map: HashMap<u32, UnplacedDetailInfo> = HashMap::new();
    
    // Инициализируем unplaced_details_map всеми деталями из запроса
    for detail in details {
        let detail_id = detail.id;
        unplaced_details_map.insert(detail_id, UnplacedDetailInfo {
            id: detail.id,
            name: detail.name.clone(),
            width: detail.width,
            length: detail.length,
            angle: detail.angle.unwrap_or(0),
            quantity: detail.quantity,
        });
    }
    
    // Размещаем детали на виртуальном листе, чтобы определить неразмещенные
    let sheet = &request.sheet;
    let (mut current_x, mut current_y) = (padding, padding);
    let (x_step, y_step) = (1, 1);
    
    // Выбираем метод раскроя
    match layout.method.as_str() {
        "horizontal" => horizontal_layout(
            details, sheet, layout, padding, x_step, y_step, 
            &mut current_x, &mut current_y, &mut details_data, &mut unplaced_details_map
        ),
        "vertical" => vertical_layout(
            details, sheet, layout, padding, x_step, y_step, 
            &mut current_x, &mut current_y, &mut details_data, &mut unplaced_details_map
        ),
        "optimal" => optimal_layout_stub(
            details, sheet, layout, padding, &mut details_data, &mut unplaced_details_map
        ),
        _ => horizontal_layout(
            details, sheet, layout, padding, x_step, y_step, 
            &mut current_x, &mut current_y, &mut details_data, &mut unplaced_details_map
        ),
    }
    
    // Получаем неразмещенные детали
    let unplaced_details: Vec<UnplacedDetailInfo> = unplaced_details_map
        .into_iter()
        .filter(|(_, detail)| detail.quantity > 0)
        .map(|(_, detail)| detail)
        .collect();
    
    // Если нет неразмещенных деталей, возвращаем пустой SVG
    if unplaced_details.is_empty() {
        return format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="400" height="100" viewBox="0 0 400 100" xmlns="http://www.w3.org/2000/svg">
<text x="200" y="50" font-family="Arial, sans-serif" font-size="14px" text-anchor="middle">Все детали размещены</text>
</svg>"#
        );
    }
    
    // Определяем размеры SVG для неразмещенных деталей
    // Упрощенный расчет - делаем компактную сетку
    let max_width = unplaced_details.iter().map(|d| d.width).max().unwrap_or(100);
    let max_length = unplaced_details.iter().map(|d| d.length).max().unwrap_or(100);
    let items_per_row = 3; // Сколько деталей в ряду
    let total_items = unplaced_details.len();
    let rows = (total_items + items_per_row - 1) / items_per_row; // Округление вверх
    
    let svg_width = (max_width + padding * 2) * items_per_row as u32;
    let svg_height = (max_length + padding * 2) * rows as u32 + padding * 2;
    
    // Создаем SVG
    let mut svg = String::new();
    
    // Объявление XML и элемента SVG
    writeln!(
        &mut svg,
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="{}" height="{}" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
        svg_width, svg_height, svg_width, svg_height
    ).unwrap();
    
    // Стили
    writeln!(
        &mut svg,
        r#"<style>
    .unplaced {{ fill: #ffa0a0; stroke: #000000; stroke-width: 1; }}
    .text {{ font-family: Arial, sans-serif; font-size: 14px; text-anchor: middle; }}
    .title {{ font-family: Arial, sans-serif; font-size: 16px; font-weight: bold; text-anchor: middle; }}
</style>"#
    ).unwrap();
    
    // Заголовок
    writeln!(
        &mut svg,
        r#"<text class="title" x="{}" y="{}">Неразмещенные детали</text>"#,
        svg_width / 2, padding
    ).unwrap();
    
    // Размещаем неразмещенные детали в сетку
    let mut x = padding;
    let mut y = padding * 2 + 20; // Отступ для заголовка
    
    for (i, unplaced) in unplaced_details.iter().enumerate() {
        // Рисуем деталь
        writeln!(
            &mut svg,
            r#"<rect class="unplaced" x="{}" y="{}" width="{}" height="{}" />"#,
            x, y, unplaced.width, unplaced.length
        ).unwrap();
        
        // Добавляем текст
        writeln!(
            &mut svg,
            r#"<text class="text" x="{}" y="{}">{} ({}x{})</text>"#,
            x + unplaced.width / 2,
            y + unplaced.length / 2,
            unplaced.name,
            unplaced.quantity, unplaced.id
        ).unwrap();
        
        // Переходим к следующей позиции
        if (i + 1) % items_per_row == 0 {
            x = padding;
            y += max_length + padding * 2;
        } else {
            x += max_width + padding * 2;
        }
    }
    
    // Закрываем SVG
    writeln!(&mut svg, "</svg>").unwrap();
    
    svg
}

// Функция для горизонтального размещения деталей
fn horizontal_layout(
    details: &[DetailDimension],
    sheet: &SheetDimension,
    layout: &LayoutOptions,
    padding: u32,
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
    let mut sorted_details = details.to_vec();
    sorted_details.sort_by(|a, b| {
        let area_a = a.width * a.length;
        let area_b = b.width * b.length;
        area_b.cmp(&area_a)
    });

    for detail in &sorted_details {
        let detail_id = detail.id;
        let mut placed_count = 0;
        
        // Попытка разместить каждый экземпляр детали
        for _ in 0..detail.quantity {
            let effective_width = detail.width;
            let effective_length = detail.length;
            let angle = detail.angle.unwrap_or(0);
            
            // Проверка, помещается ли деталь в оставшейся ширине
            let next_x = if x_step > 0 {
                *current_x as i32 + effective_width as i32 + gap as i32 + blade_width as i32
            } else {
                *current_x as i32 - effective_width as i32 - gap as i32 - blade_width as i32
            };
            
            if (x_step > 0 && next_x > (sheet.width + padding) as i32) ||
               (x_step < 0 && next_x < padding as i32) {
                // Если не помещается, переходим на новую строку
                *current_x = if x_step > 0 { padding } else { sheet.width };
                *current_y = if y_step > 0 {
                    *current_y + gap + blade_width + (&sorted_details).iter().map(|d| d.length).max().unwrap_or(0)
                } else {
                    let max_length = (&sorted_details).iter().map(|d| d.length).max().unwrap_or(0) as i32;
                    (*current_y as i32 - gap as i32 - blade_width as i32 - max_length).max(0) as u32
                };
            }

            // Проверка, помещается ли деталь по высоте
            if (y_step > 0 && (*current_y + effective_length) > sheet.length + padding) ||
               (y_step < 0 && (*current_y as i32 - effective_length as i32) < padding as i32) {
                // Если деталь не помещается по высоте, она не может быть размещена
                continue;
            }

            // Добавляем информацию о детали
            let x_pos = if x_step < 0 { 
                (*current_x as i32 - effective_width as i32).max(0) as u32 
            } else { 
                *current_x 
            };
            
            let y_pos = if y_step < 0 { 
                (*current_y as i32 - effective_length as i32).max(0) as u32 
            } else { 
                *current_y 
            };
            
            details_data.push(DetailInfo {
                id: detail.id,
                name: detail.name.clone(),
                width: effective_width,
                length: effective_length,
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
                let new_x = (*current_x as i32 - effective_width as i32 - gap as i32 - blade_width as i32).max(0);
                *current_x = new_x as u32;
            }
        }
        
        // Обновляем unplaced_details_map с количеством деталей, которые не удалось разместить
        if let Some(unplaced_detail) = unplaced_details_map.get_mut(&detail_id) {
            unplaced_detail.quantity = detail.quantity - placed_count;
        }
    }
}

// Функция для вертикального размещения деталей
fn vertical_layout(
    details: &[DetailDimension],
    sheet: &SheetDimension,
    layout: &LayoutOptions,
    padding: u32,
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
    let mut sorted_details = details.to_vec();
    sorted_details.sort_by(|a, b| {
        let area_a = a.width * a.length;
        let area_b = b.width * b.length;
        area_b.cmp(&area_a)
    });

    for detail in &sorted_details {
        let detail_id = detail.id;
        let mut placed_count = 0;
        
        // Попытка разместить каждый экземпляр детали
        for _ in 0..detail.quantity {
            let effective_width = detail.width;
            let effective_length = detail.length;
            let angle = detail.angle.unwrap_or(0);
            
            // Проверка, помещается ли деталь в оставшейся высоте
            let next_y = if y_step > 0 {
                *current_y as i32 + effective_length as i32 + gap as i32 + blade_width as i32
            } else {
                *current_y as i32 - effective_length as i32 - gap as i32 - blade_width as i32
            };
            
            if (y_step > 0 && next_y > (sheet.length + padding) as i32) ||
               (y_step < 0 && next_y < padding as i32) {
                // Если не помещается, переходим на новый столбец
                *current_y = if y_step > 0 { padding } else { sheet.length };
                *current_x = if x_step > 0 {
                    *current_x + gap + blade_width + (&sorted_details).iter().map(|d| d.width).max().unwrap_or(0)
                } else {
                    let max_width = (&sorted_details).iter().map(|d| d.width).max().unwrap_or(0) as i32;
                    (*current_x as i32 - gap as i32 - blade_width as i32 - max_width).max(0) as u32
                };
            }

            // Проверка, помещается ли деталь по ширине
            if (x_step > 0 && (*current_x + effective_width) > sheet.width + padding) ||
               (x_step < 0 && (*current_x as i32 - effective_width as i32) < padding as i32) {
                // Если деталь не помещается по ширине, она не может быть размещена
                continue;
            }

            // Добавляем информацию о детали
            let x_pos = if x_step < 0 { 
                (*current_x as i32 - effective_width as i32).max(0) as u32 
            } else { 
                *current_x 
            };
            
            let y_pos = if y_step < 0 { 
                (*current_y as i32 - effective_length as i32).max(0) as u32 
            } else { 
                *current_y 
            };
            
            details_data.push(DetailInfo {
                id: detail.id,
                name: detail.name.clone(),
                width: effective_width,
                length: effective_length,
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
            if y_step > 0 {
                *current_y += effective_length + gap + blade_width;
            } else {
                let new_y = (*current_y as i32 - effective_length as i32 - gap as i32 - blade_width as i32).max(0);
                *current_y = new_y as u32;
            }
        }
        
        // Обновляем unplaced_details_map с количеством деталей, которые не удалось разместить
        if let Some(unplaced_detail) = unplaced_details_map.get_mut(&detail_id) {
            unplaced_detail.quantity = detail.quantity - placed_count;
        }
    }
}

// Заглушка для optimal_layout - упрощенное размещение
fn optimal_layout_stub(
    details: &[DetailDimension],
    sheet: &SheetDimension,
    layout: &LayoutOptions,
    padding: u32,
    details_data: &mut Vec<DetailInfo>,
    unplaced_details_map: &mut HashMap<u32, UnplacedDetailInfo>,
) {
    // Заглушка вызывает горизонтальное размещение как временное решение
    // В реальной имплементации здесь должен быть оптимальный алгоритм упаковки
    
    // Инициализируем начальную позицию
    let mut current_x = padding;
    let mut current_y = padding;
    
    // Направление движения
    let x_step = 1;
    let y_step = 1;
    
    // Используем горизонтальное размещение как временное решение
    horizontal_layout(
        details, sheet, layout, padding, x_step, y_step, 
        &mut current_x, &mut current_y, details_data, unplaced_details_map
    );
}

// Функция для расчета статистики
fn calculate_statistics(
    sheet: &SheetDimension,
    details: &[DetailInfo],
    unplaced_details: &[UnplacedDetailInfo],
) -> Statistics {
    // Используем структуру Statistics из модуля models
    
    let sheet_area = sheet.width * sheet.length;
    
    // Площадь используемых деталей
    let used_area = details.iter()
        .map(|detail| detail.width * detail.length)
        .sum();
    
    // Площадь отходов
    let waste_area = sheet_area - used_area;
    
    // Общая длина реза (приблизительно)
    let cut_length = details.iter()
        .map(|detail| 2 * (detail.width + detail.length))
        .sum();
    
    // Общая длина кромки (периметр всех деталей)
    let edge_length = details.iter()
        .map(|detail| 2 * (detail.width + detail.length))
        .sum();
    
    // Количество размещенных деталей
    let detail_count = details.len() as u32;
    
    // Количество неразмещенных деталей
    let unplaced_count = unplaced_details.iter()
        .map(|detail| detail.quantity)
        .sum::<u32>();
    
    // Эффективность
    let efficiency = if sheet_area > 0 {
        (used_area as f32 / sheet_area as f32) * 100.0
    } else {
        0.0
    };
    
    Statistics {
        sheet_area,
        used_area,
        waste_area,
        cut_length,
        edge_length,
        detail_count,
        unplaced_count,
        efficiency,
    }
}