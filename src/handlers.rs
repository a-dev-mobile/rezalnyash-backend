use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use log::info;

use crate::app_state::AppState;
use crate::models::SheetRequest;
use crate::svg_generator::generate_svg;

// Обработчик для POST запроса JSON
pub async fn create_sheet(
    State(state): State<AppState>,
    Json(request): Json<SheetRequest>,
) -> impl IntoResponse {
    info!("Получен запрос на раскрой: {:?}", request);

    let mut sheet_data = state.sheet_data.lock().unwrap();
    *sheet_data = Some(request.clone());

    // Формируем SVG с размерами листа и деталями
    let svg = generate_svg(&request);

    // Возвращаем SVG
    (StatusCode::OK, [("Content-Type", "image/svg+xml")], svg)
}

// Обработчик для GET запроса
pub async fn get_sheet(State(state): State<AppState>) -> Response {
    let sheet_data = state.sheet_data.lock().unwrap();

    match &*sheet_data {
        Some(data) => {
            let svg = generate_svg(&data);
            (StatusCode::OK, [("Content-Type", "image/svg+xml")], svg).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            "Нет данных о раскрое. Пожалуйста, сначала отправьте POST-запрос.",
        )
            .into_response(),
    }
}