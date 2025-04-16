use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use log::{info, error};
use serde_json::Value;

use crate::app_state::AppState;
use crate::models::SheetRequest;

use crate::svg_generator::{generate_sheet_svg, generate_unplaced_svg};





// Обработчик для POST запроса, возвращающий SVG с раскроем листа
pub async fn create_sheet_svg(
    State(state): State<AppState>,
    Json(request): Json<SheetRequest>,
) -> impl IntoResponse {
    info!("Получен запрос на раскрой (SVG): {:?}", request);

    let mut sheet_data = state.sheet_data.lock().unwrap();
    *sheet_data = Some(request.clone());

    // Генерируем SVG с раскроем листа
    let svg_data = generate_sheet_svg(&request);
    
    // Возвращаем SVG с правильным Content-Type
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/svg+xml")],
        svg_data,
    )
        .into_response()
}

// Обработчик для GET запроса, возвращающий SVG с раскроем листа
pub async fn get_sheet_svg(State(state): State<AppState>) -> Response {
    let sheet_data = state.sheet_data.lock().unwrap();

    match &*sheet_data {
        Some(data) => {
            // Генерируем SVG с раскроем листа
            let svg_data = generate_sheet_svg(data);
            
            // Возвращаем SVG с правильным Content-Type
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "image/svg+xml")],
                svg_data,
            )
                .into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/plain")],
            "Нет данных о раскрое. Пожалуйста, сначала отправьте POST-запрос.".to_string(),
        )
            .into_response(),
    }
}

// Новый обработчик для GET запроса, возвращающий SVG с неразмещенными деталями
pub async fn get_unplaced_svg(State(state): State<AppState>) -> Response {
    let sheet_data = state.sheet_data.lock().unwrap();

    match &*sheet_data {
        Some(data) => {
            // Генерируем SVG с неразмещенными деталями
            let svg_data = generate_unplaced_svg(data);
            
            // Возвращаем SVG с правильным Content-Type
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "image/svg+xml")],
                svg_data,
            )
                .into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/plain")],
            "Нет данных о раскрое. Пожалуйста, сначала отправьте POST-запрос.".to_string(),
        )
            .into_response(),
    }
}