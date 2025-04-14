use crate::models::SheetRequest;

// Функция для генерации SVG на основе размеров листа и деталей
pub fn generate_svg(request: &SheetRequest) -> String {
    let sheet = &request.sheet;
    let details = &request.details;

    // Параметры расположения (по умолчанию - горизонтально с зазором 5)
    let orientation = match &request.layout {
        Some(layout) => &layout.orientation,
        None => "horizontal",
    };

    let gap = match &request.layout {
        Some(layout) => layout.gap,
        None => 5,
    };

    // Добавляем отступ в 10 единиц со всех сторон
    let padding = 10;
    let view_width = sheet.width + 2 * padding;
    let view_height = sheet.length + 2 * padding;

    // Начинаем создавать SVG
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {view_width} {view_height}" width="{view_width}" height="{view_height}">
            <style>
                .sheet {{ fill: #f0f0f0; stroke: #333; stroke-width: 2; }}
                .detail {{ fill: #000; stroke: #333; stroke-width: 1; opacity: 0.7; }}
                .detail-text {{ font-family: Arial; font-size: 12px; fill: white; text-anchor: middle; }}
                .text {{ font-family: Arial; font-size: 14px; text-anchor: middle; }}
                .title {{ font-family: Arial; font-size: 18px; font-weight: bold; text-anchor: middle; }}
            </style>
            <rect x="{padding}" y="{padding}" width="{}" height="{}" class="sheet"/>
            <text x="{}" y="{}" class="text">{} x {}</text>
            <text x="{}" y="{}" class="title">РезальНяш</text>"#,
        sheet.width,
        sheet.length,
        sheet.width / 2 + padding,
        sheet.length / 2 + padding,
        sheet.width,
        sheet.length,
        view_width / 2,
        padding / 2 + 9
    );

    // Добавляем детали в SVG
    let mut current_x = padding;
    let mut current_y = padding;

    for detail in details {
        if orientation == "horizontal" {
            // Проверяем, помещается ли деталь в оставшуюся ширину листа
            if current_x + detail.width > sheet.width + padding {
                // Если не помещается, переходим на новую строку
                current_x = padding;
                current_y += gap + details.iter().map(|d| d.length).max().unwrap_or(0);
            }

            // Проверяем, помещается ли деталь по высоте
            if current_y + detail.length > sheet.length + padding {
                // Если деталь не помещается по высоте, прекращаем добавление
                break;
            }

            // Добавляем деталь
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" class="detail" />
                <text x="{}" y="{}" class="detail-text">ID:{}</text>"#,
                current_x,
                current_y,
                detail.width,
                detail.length,
                current_x + detail.width / 2,
                current_y + detail.length / 2,
                detail.id
            ));

            // Перемещаем указатель для следующей детали
            current_x += detail.width + gap;
        } else {
            // vertical
            // Проверяем, помещается ли деталь в оставшуюся высоту листа
            if current_y + detail.length > sheet.length + padding {
                // Если не помещается, переходим на новый столбец
                current_y = padding;
                current_x += gap + details.iter().map(|d| d.width).max().unwrap_or(0);
            }

            // Проверяем, помещается ли деталь по ширине
            if current_x + detail.width > sheet.width + padding {
                // Если деталь не помещается по ширине, прекращаем добавление
                break;
            }

            // Добавляем деталь
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" class="detail" />
                <text x="{}" y="{}" class="detail-text">ID:{}</text>"#,
                current_x,
                current_y,
                detail.width,
                detail.length,
                current_x + detail.width / 2,
                current_y + detail.length / 2,
                detail.id
            ));

            // Перемещаем указатель для следующей детали
            current_y += detail.length + gap;
        }
    }

    // Закрываем SVG
    svg.push_str("</svg>");

    svg
}