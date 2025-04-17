// svg_builder.rs - Модуль для формирования SVG-документов
//
// Предоставляет удобный API для создания SVG-документов
// с раскроем листовых материалов и размещенными деталями.

use std::fmt::Write;

use crate::models::other::{DetailInfo, UnplacedDetailInfo};

/// Структура для построения SVG-документов
pub struct SvgBuilder {
    /// Ширина SVG документа
    width: u32,

    /// Высота SVG документа
    height: u32,

    /// Содержимое SVG документа
    content: String,

    /// Добавлять ли сетку на SVG
    show_grid: bool,

    /// Шаг сетки в пикселях
    grid_step: u32,
}

impl SvgBuilder {
    /// Создает новый экземпляр SvgBuilder
    pub fn new(width: u32, height: u32) -> Self {
        let mut builder = SvgBuilder {
            width,
            height,
            content: String::new(),
            show_grid: false,
            grid_step: 50,
        };

        // Инициализируем XML-заголовок и корневой элемент SVG
        writeln!(
            &mut builder.content,
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="{}" height="{}" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height, width, height
        )
        .unwrap();

        // Добавляем стили
        builder.add_styles();

        builder
    }

    /// Добавляет стили CSS в SVG документ
    fn add_styles(&mut self) {
        writeln!(
            &mut self.content,
            r#"<style>
    /* Стиль для листа материала */
    .sheet {{ fill: #f0f0f0; stroke: #000000; stroke-width: 2; }}
    
    /* Стиль для размещенных деталей */
    .detail {{ fill: #a0d0ff; stroke: #000000; stroke-width: 1; }}
    
    /* Стиль для текста идентификатора детали */
    .text {{ font-family: Arial, sans-serif; font-size: 14px; text-anchor: middle; dominant-baseline: middle; }}
    
    /* Стиль для индикаторов отступов */
    .margin-indicator {{ stroke: #888888; stroke-width: 1; stroke-dasharray: 5,5; fill: none; }}
    
    /* Стиль для заголовка */
    .title {{ font-family: Arial, sans-serif; font-size: 16px; font-weight: bold; text-anchor: middle; }}
    
    /* Стиль для размеров */
    .dimension {{ font-family: Arial, sans-serif; font-size: 10px; fill: #444444; }}
    
    /* Стиль для линии разреза */
    .cut-line {{ stroke: #ff0000; stroke-width: 0.5; stroke-dasharray: 5,3; }}
    
    /* Стиль для сетки */
    .grid {{ stroke: #cccccc; stroke-width: 0.5; opacity: 0.5; }}
    
    /* Стиль для границы области */
    .boundary {{ stroke: #000000; stroke-width: 1; fill: none; }}
</style>"#
        ).unwrap();
    }

    /// Включает отображение сетки
    pub fn with_grid(&mut self, step: u32) -> &mut Self {
        self.show_grid = true;
        self.grid_step = step;
        self
    }

    /// Добавляет сетку в SVG документ
    pub fn add_grid(&mut self) -> &mut Self {
        if !self.show_grid {
            return self;
        }

        writeln!(&mut self.content, r#"<!-- Сетка -->"#).unwrap();

        // Добавляем горизонтальные линии
        for y in (0..=self.height).step_by(self.grid_step as usize) {
            writeln!(
                &mut self.content,
                r#"<line class="grid" x1="0" y1="{}" x2="{}" y2="{}" />"#,
                y, self.width, y
            )
            .unwrap();
        }

        // Добавляем вертикальные линии
        for x in (0..=self.width).step_by(self.grid_step as usize) {
            writeln!(
                &mut self.content,
                r#"<line class="grid" x1="{}" y1="0" x2="{}" y2="{}" />"#,
                x, x, self.height
            )
            .unwrap();
        }

        self
    }

    /// Добавляет лист материала
    pub fn add_sheet(&mut self, x: u32, y: u32, width: u32, height: u32) -> &mut Self {
        writeln!(
            &mut self.content,
            r#"<!-- Лист материала -->
<rect class="sheet" x="{}" y="{}" width="{}" height="{}" />"#,
            x, y, width, height
        )
        .unwrap();

        self
    }

    /// Добавляет индикаторы отступов внутри контура листа
    pub fn add_margin_indicators(
        &mut self,
        sheet_x: u32,
        sheet_y: u32,
        sheet_width: u32,
        sheet_height: u32,
        margins: &crate::models::margins::Margins,
    ) -> &mut Self {
        writeln!(
            &mut self.content,
            r#"<!-- Индикаторы отступов внутри листа -->"#
        )
        .unwrap();

        // Рисуем внутренний прямоугольник, учитывающий отступы
        writeln!(
            &mut self.content,
            r#"<rect class="margin-indicator" x="{}" y="{}" width="{}" height="{}" />"#,
            margins.left,
            margins.top,
            sheet_width - margins.right - margins.left,
            sheet_height - margins.bottom - margins.top,
        )
        .unwrap();

        self
    }

    /// Добавляет размещенную деталь
    pub fn add_detail(&mut self, detail: &DetailInfo) -> &mut Self {
        // Определяем цвет детали на основе её ID
        let hue = (detail.id * 60) % 360;

        writeln!(
            &mut self.content,
            r#"<!-- Деталь {} ({}) -->
<g>"#,
            detail.name, detail.id
        )
        .unwrap();

        // Рисуем прямоугольник детали
        writeln!(
            &mut self.content,
            r#"<rect class="detail" x="{}" y="{}" width="{}" height="{}" style="fill: hsl({}, 70%, 80%);" />"#,
            detail.x, detail.y, detail.width, detail.height, hue
        ).unwrap();

        // Добавляем текст детали
        writeln!(
            &mut self.content,
            r#"<text class="text" x="{}" y="{}">{} ({})</text>"#,
            detail.textPosition.x, detail.textPosition.y, detail.name, detail.id
        )
        .unwrap();

        // Добавляем размеры детали
        self.add_dimension_text(
            detail.x + detail.width / 2,
            detail.y - 5,
            &format!("{}мм", detail.width),
            "dimension",
        );

        self.add_dimension_text(
            detail.x - 5,
            detail.y + detail.height / 2,
            &format!("{}мм", detail.height),
            "dimension",
        );

        writeln!(&mut self.content, r#"</g>"#).unwrap();

        self
    }

    /// Добавляет заголовок
    pub fn add_title(&mut self, title: &str, x: u32, y: u32) -> &mut Self {
        writeln!(
            &mut self.content,
            r#"<!-- Заголовок -->
<text class="title" x="{}" y="{}">{}</text>"#,
            x, y, title
        )
        .unwrap();

        self
    }

    /// Добавляет текст размеров
    pub fn add_dimension_text(&mut self, x: u32, y: u32, text: &str, class: &str) -> &mut Self {
        writeln!(
            &mut self.content,
            r#"<text class="{}" x="{}" y="{}" text-anchor="middle">{}</text>"#,
            class, x, y, text
        )
        .unwrap();

        self
    }

    /// Добавляет линию разреза
    pub fn add_cut_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) -> &mut Self {
        writeln!(
            &mut self.content,
            r#"<line class="cut-line" x1="{}" y1="{}" x2="{}" y2="{}" />"#,
            x1, y1, x2, y2
        )
        .unwrap();

        self
    }

    /// Добавляет комментарий в SVG документ
    pub fn add_comment(&mut self, comment: &str) -> &mut Self {
        writeln!(&mut self.content, r#"<!-- {} -->"#, comment).unwrap();

        self
    }

    /// Добавляет группу элементов с трансформацией
    pub fn begin_group(&mut self, transform: Option<&str>) -> &mut Self {
        match transform {
            Some(t) => writeln!(&mut self.content, r#"<g transform="{}">"#, t).unwrap(),
            None => writeln!(&mut self.content, r#"<g>"#).unwrap(),
        }

        self
    }

    /// Закрывает группу элементов
    pub fn end_group(&mut self) -> &mut Self {
        writeln!(&mut self.content, r#"</g>"#).unwrap();

        self
    }

    /// Добавляет произвольный SVG-элемент
    pub fn add_raw_svg(&mut self, svg_content: &str) -> &mut Self {
        writeln!(&mut self.content, "{}", svg_content).unwrap();

        self
    }

    /// Добавляет произвольный текст в SVG
    pub fn add_text(&mut self, x: u32, y: u32, text: &str, class: &str) -> &mut Self {
        writeln!(
            &mut self.content,
            r#"<text class="{}" x="{}" y="{}">{}</text>"#,
            class, x, y, text
        )
        .unwrap();

        self
    }

    /// Добавляет прямоугольник с заданными свойствами
    pub fn add_rect(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        class: &str,
        extra_attrs: Option<&str>,
    ) -> &mut Self {
        match extra_attrs {
            Some(attrs) => writeln!(
                &mut self.content,
                r#"<rect class="{}" x="{}" y="{}" width="{}" height="{}" {} />"#,
                class, x, y, width, height, attrs
            )
            .unwrap(),
            None => writeln!(
                &mut self.content,
                r#"<rect class="{}" x="{}" y="{}" width="{}" height="{}" />"#,
                class, x, y, width, height
            )
            .unwrap(),
        }

        self
    }

    /// Добавляет линию с заданными свойствами
    pub fn add_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, class: &str) -> &mut Self {
        writeln!(
            &mut self.content,
            r#"<line class="{}" x1="{}" y1="{}" x2="{}" y2="{}" />"#,
            class, x1, y1, x2, y2
        )
        .unwrap();

        self
    }

    /// Добавляет окружность с заданными свойствами
    pub fn add_circle(&mut self, cx: u32, cy: u32, r: u32, class: &str) -> &mut Self {
        writeln!(
            &mut self.content,
            r#"<circle class="{}" cx="{}" cy="{}" r="{}" />"#,
            class, cx, cy, r
        )
        .unwrap();

        self
    }

    /// Строит итоговый SVG-документ
    pub fn build(&mut self) -> String {
        // Если нужно, добавляем сетку
        if self.show_grid {
            self.add_grid();
        }

        // Закрываем корневой элемент SVG
        writeln!(&mut self.content, "</svg>").unwrap();

        // Возвращаем содержимое SVG
        self.content.clone()
    }
}
