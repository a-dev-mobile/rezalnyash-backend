use serde_json::{json, Value};
use crate::models::{DetailInfo, LayoutInfo, SheetInfo, SheetRequest, SheetResponse, Statistics, TextPosition, UnplacedDetailInfo, ViewBox};

// Функция для генерации JSON-данных на основе размеров листа и деталей
pub fn generate_json(request: &SheetRequest) -> Value {
    let sheet = &request.sheet;
    let details = &request.details;
    let layout = &request.layout;

    // Use margin from request parameters
    let padding = layout.margin;
    
    // Sheet information
    let sheet_info = SheetInfo {
        width: sheet.width,
        length: sheet.length,
        padding,
        viewBox: ViewBox {
            width: sheet.width + 2 * padding,
            height: sheet.length + 2 * padding
        }
    };

    // Layout information
    let layout_info = LayoutInfo {
        method: layout.method.clone(),
        gap: layout.gap,
        blade_width: layout.blade_width,
        margin: layout.margin,
        starting_corner: layout.starting_corner.clone()
    };

    // Track placed and unplaced details
    let mut details_data = Vec::new();
    let mut unplaced_details_map: std::collections::HashMap<u32, UnplacedDetailInfo> = std::collections::HashMap::new();
    
    // Initialize unplaced_details_map with all details from the request
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
    
    // Define starting position based on starting_corner
    let (mut current_x, mut current_y) = match layout.starting_corner.as_str() {
        "top-left" => (padding, padding),
        "top-right" => (sheet.width, padding),
        "bottom-left" => (padding, sheet.length),
        "bottom-right" => (sheet.width, sheet.length),
        _ => (padding, padding), // Default to top-left
    };

    // Movement direction depends on starting_corner
    let (x_step, y_step) = match layout.starting_corner.as_str() {
        "top-left" => (1, 1),
        "top-right" => (-1, 1),
        "bottom-left" => (1, -1),
        "bottom-right" => (-1, -1),
        _ => (1, 1), // Default to top-left
    };

    // Calculate positions based on layout method
    match layout.method.as_str() {
        "horizontal" => horizontal_layout(
            details, sheet, layout, padding, x_step, y_step, 
            &mut current_x, &mut current_y, &mut details_data, &mut unplaced_details_map
        ),
        "vertical" => vertical_layout(
            details, sheet, layout, padding, x_step, y_step, 
            &mut current_x, &mut current_y, &mut details_data, &mut unplaced_details_map
        ),
        "optimal" => optimal_layout(
            details, sheet, layout, padding, &mut details_data, &mut unplaced_details_map
        ),
        _ => horizontal_layout(
            details, sheet, layout, padding, x_step, y_step, 
            &mut current_x, &mut current_y, &mut details_data, &mut unplaced_details_map
        ),
    }

    // Convert unplaced_details_map to a vector
    let unplaced_details: Vec<UnplacedDetailInfo> = unplaced_details_map
        .into_iter()
        .filter(|(_, detail)| detail.quantity > 0)  // Only include details with quantity > 0
        .map(|(_, detail)| detail)
        .collect();

    // Calculate statistics
    let statistics = calculate_statistics(sheet, &details_data, &unplaced_details);

    // Build final JSON
    let response = SheetResponse {
        sheet: sheet_info,
        layout: layout_info,
        details: details_data,
        unplaced_details,
        statistics,
    };

    json!(response)
}

// 3. Update the layout functions to track unplaced details

// For horizontal_layout function
fn horizontal_layout(
    details: &[crate::models::DetailDimension],
    sheet: &crate::models::SheetDimension,
    layout: &crate::models::LayoutOptions,
    padding: u32,
    x_step: i32,
    y_step: i32,
    current_x: &mut u32,
    current_y: &mut u32,
    details_data: &mut Vec<DetailInfo>,
    unplaced_details_map: &mut std::collections::HashMap<u32, UnplacedDetailInfo>,
) {
    let gap = layout.gap;
    let blade_width = layout.blade_width;

    for detail in details {
        let detail_id = detail.id;
        let mut placed_count = 0;
        
        // Try to place each instance of the detail
        for _ in 0..detail.quantity {
            let effective_width = detail.width;
            let effective_length = detail.length;
            let angle = detail.angle.unwrap_or(0);
            
            // Check if detail fits in remaining width
            let next_x = if x_step > 0 {
                *current_x as i32 + effective_width as i32 + gap as i32 + blade_width as i32
            } else {
                *current_x as i32 - effective_width as i32 - gap as i32 - blade_width as i32
            };
            
            if (x_step > 0 && next_x > (sheet.width + padding) as i32) ||
               (x_step < 0 && next_x < padding as i32) {
                // If it doesn't fit, move to a new row
                *current_x = if x_step > 0 { padding } else { sheet.width };
                *current_y = if y_step > 0 {
                    *current_y + gap + blade_width + details.iter().map(|d| d.length).max().unwrap_or(0)
                } else {
                    let max_length = details.iter().map(|d| d.length).max().unwrap_or(0) as i32;
                    (*current_y as i32 - gap as i32 - blade_width as i32 - max_length).max(0) as u32
                };
            }

            // Check if detail fits in height
            if (y_step > 0 && (*current_y + effective_length) > sheet.length + padding) ||
               (y_step < 0 && (*current_y as i32 - effective_length as i32) < padding as i32) {
                // If the detail doesn't fit in height, it can't be placed
                continue;
            }

            // Add the detail info
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

            // Move pointer for next detail
            if x_step > 0 {
                *current_x += effective_width + gap + blade_width;
            } else {
                let new_x = (*current_x as i32 - effective_width as i32 - gap as i32 - blade_width as i32).max(0);
                *current_x = new_x as u32;
            }
        }
        
        // Update unplaced_details_map with the number of details that couldn't be placed
        if let Some(unplaced_detail) = unplaced_details_map.get_mut(&detail_id) {
            unplaced_detail.quantity = detail.quantity - placed_count;
        }
    }
}

// Similar updates for vertical_layout function
fn vertical_layout(
    details: &[crate::models::DetailDimension],
    sheet: &crate::models::SheetDimension,
    layout: &crate::models::LayoutOptions,
    padding: u32,
    x_step: i32,
    y_step: i32,
    current_x: &mut u32,
    current_y: &mut u32,
    details_data: &mut Vec<DetailInfo>,
    unplaced_details_map: &mut std::collections::HashMap<u32, UnplacedDetailInfo>,
) {
    let gap = layout.gap;
    let blade_width = layout.blade_width;

    for detail in details {
        let detail_id = detail.id;
        let mut placed_count = 0;
        
        // Try to place each instance of the detail
        for _ in 0..detail.quantity {
            let effective_width = detail.width;
            let effective_length = detail.length;
            let angle = detail.angle.unwrap_or(0);
            
            // Check if detail fits in remaining height
            let next_y = if y_step > 0 {
                *current_y as i32 + effective_length as i32 + gap as i32 + blade_width as i32
            } else {
                *current_y as i32 - effective_length as i32 - gap as i32 - blade_width as i32
            };
            
            if (y_step > 0 && next_y > (sheet.length + padding) as i32) ||
               (y_step < 0 && next_y < padding as i32) {
                // If it doesn't fit, move to a new column
                *current_y = if y_step > 0 { padding } else { sheet.length };
                *current_x = if x_step > 0 {
                    *current_x + gap + blade_width + details.iter().map(|d| d.width).max().unwrap_or(0)
                } else {
                    let max_width = details.iter().map(|d| d.width).max().unwrap_or(0) as i32;
                    (*current_x as i32 - gap as i32 - blade_width as i32 - max_width).max(0) as u32
                };
            }

            // Check if detail fits in width
            if (x_step > 0 && (*current_x + effective_width) > sheet.width + padding) ||
               (x_step < 0 && (*current_x as i32 - effective_width as i32) < padding as i32) {
                // If the detail doesn't fit in width, it can't be placed
                continue;
            }

            // Add the detail info
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

            // Move pointer for next detail
            if y_step > 0 {
                *current_y += effective_length + gap + blade_width;
            } else {
                let new_y = (*current_y as i32 - effective_length as i32 - gap as i32 - blade_width as i32).max(0);
                *current_y = new_y as u32;
            }
        }
        
        // Update unplaced_details_map with the number of details that couldn't be placed
        if let Some(unplaced_detail) = unplaced_details_map.get_mut(&detail_id) {
            unplaced_detail.quantity = detail.quantity - placed_count;
        }
    }
}

// Update optimal_layout function
fn optimal_layout(
    details: &[crate::models::DetailDimension],
    sheet: &crate::models::SheetDimension,
    layout: &crate::models::LayoutOptions,
    padding: u32,
    details_data: &mut Vec<DetailInfo>,
    unplaced_details_map: &mut std::collections::HashMap<u32, UnplacedDetailInfo>,
) {
    // Sort details by area (largest to smallest)
    let mut sorted_details = details.to_vec();
    sorted_details.sort_by(|a, b| {
        let area_a = a.width * a.length;
        let area_b = b.width * b.length;
        area_b.cmp(&area_a)
    });

    // Create array of occupied areas
    let mut occupied = vec![vec![false; (sheet.length + 2 * padding) as usize]; (sheet.width + 2 * padding) as usize];

    for detail in sorted_details {
        let detail_id = detail.id;
        let mut placed_count = 0;
        
        // Try to place each instance of the detail
        for _ in 0..detail.quantity {
            // Check all possible positions for the detail
            if let Some((best_x, best_y, best_angle)) = find_best_position(
                &detail, sheet, layout, padding, &mut occupied
            ) {
                // Mark area as occupied
                mark_as_occupied(
                    best_x, best_y, detail.width, detail.length, 
                    best_angle, layout.gap, &mut occupied
                );

                // Add detail info
                details_data.push(DetailInfo {
                    id: detail.id,
                    name: detail.name.clone(),
                    width: detail.width,
                    length: detail.length,
                    angle: best_angle,
                    x: best_x,
                    y: best_y,
                    textPosition: TextPosition {
                        x: best_x + detail.width / 2,
                        y: best_y + detail.length / 2
                    }
                });
                
                placed_count += 1;
            }
        }
        
        // Update unplaced_details_map with the number of details that couldn't be placed
        if let Some(unplaced_detail) = unplaced_details_map.get_mut(&detail_id) {
            unplaced_detail.quantity = detail.quantity - placed_count;
        }
    }
}

// Функция для поиска лучшей позиции детали при оптимальном раскрое
fn find_best_position(
    detail: &crate::models::DetailDimension,
    sheet: &crate::models::SheetDimension,
    layout: &crate::models::LayoutOptions,
    padding: u32,
    occupied: &mut Vec<Vec<bool>>,
) -> Option<(u32, u32, u32)> {
    // Проверяем все возможные положения
    let mut best_position: Option<(u32, u32, u32)> = None;
    let mut best_metric = std::u32::MAX;

    // Перебираем возможные углы поворота
    let possible_angles = if detail.angle.is_some() {
        vec![detail.angle.unwrap()]
    } else {
        vec![0, 90] // По умолчанию проверяем 0 и 90 градусов
    };

    for angle in possible_angles {
        // Определяем фактические размеры с учетом поворота
        let (width, length) = if angle == 90 {
            (detail.length, detail.width)
        } else {
            (detail.width, detail.length)
        };

        for y in padding..(sheet.length + padding) {
            for x in padding..(sheet.width + padding) {
                // Проверяем, помещается ли деталь на листе
                if x + width > sheet.width + padding || y + length > sheet.length + padding {
                    continue;
                }

                // Проверяем, не пересекается ли с другими деталями
                let mut can_place = true;
                for check_y in y..(y + length + layout.gap) {
                    for check_x in x..(x + width + layout.gap) {
                        if check_y < occupied.len() as u32 && check_x < occupied[0].len() as u32 && 
                           occupied[check_x as usize][check_y as usize] {
                            can_place = false;
                            break;
                        }
                    }
                    if !can_place {
                        break;
                    }
                }

                if can_place {
                    // Вычисляем метрику для этой позиции (расстояние от начала координат)
                    let metric = x * x + y * y;
                    if metric < best_metric {
                        best_metric = metric;
                        best_position = Some((x, y, angle));
                    }
                }
            }
        }
    }

    best_position
}

// Помечаем область как занятую
fn mark_as_occupied(
    x: u32, y: u32, width: u32, length: u32, 
    angle: u32, gap: u32, occupied: &mut Vec<Vec<bool>>
) {
    let (width, length) = if angle == 90 {
        (length, width)
    } else {
        (width, length)
    };

    for mark_y in y..(y + length + gap) {
        for mark_x in x..(x + width + gap) {
            if mark_y < occupied.len() as u32 && mark_x < occupied[0].len() as u32 {
                occupied[mark_x as usize][mark_y as usize] = true;
            }
        }
    }
}

// Функция для расчета статистики
fn calculate_statistics(
    sheet: &crate::models::SheetDimension,
    details: &[DetailInfo],
    unplaced_details: &[UnplacedDetailInfo],
) -> Statistics {
    let sheet_area = sheet.width * sheet.length;
    
    // Area of used details
    let used_area = details.iter()
        .map(|detail| detail.width * detail.length)
        .sum();
    
    // Area of waste
    let waste_area = sheet_area - used_area;
    
    // Total cut length (approximate)
    let cut_length = details.iter()
        .map(|detail| 2 * (detail.width + detail.length))
        .sum();
    
    // Total edge length (perimeter of all details)
    let edge_length = details.iter()
        .map(|detail| 2 * (detail.width + detail.length))
        .sum();
    
    // Number of placed details
    let detail_count = details.len() as u32;
    
    // Number of unplaced details
    let unplaced_count = unplaced_details.iter()
        .map(|detail| detail.quantity)
        .sum::<u32>();
    
    // Efficiency
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
        unplaced_count, // Add this field to Statistics struct
        efficiency,
    }
}