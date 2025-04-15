use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use log::info;
use serde_json::Value;

use crate::app_state::AppState;
use crate::models::SheetRequest;
use crate::json_generator::generate_json;

// Обработчик для POST запроса JSON, возвращающий данные в JSON формате
pub async fn create_sheet_json(
    State(state): State<AppState>,
    Json(request): Json<SheetRequest>,
) -> impl IntoResponse {
    info!("Получен запрос на раскрой (JSON): {:?}", request);

    let mut sheet_data = state.sheet_data.lock().unwrap();
    *sheet_data = Some(request.clone());

    // Формируем данные о раскрое в формате JSON
    let json_data = generate_json(&request);

    // Возвращаем JSON
    (StatusCode::OK, Json(json_data))
}

// Обработчик для GET запроса JSON
pub async fn get_sheet_json(State(state): State<AppState>) -> Response {
    let sheet_data = state.sheet_data.lock().unwrap();

    match &*sheet_data {
        Some(data) => {
            let json_data = generate_json(data);
            (StatusCode::OK, Json(json_data)).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(Value::String("Нет данных о раскрое. Пожалуйста, сначала отправьте POST-запрос.".to_string())),
        )
            .into_response(),
    }
}

// Обработчик для получения примера JSON-структуры запроса
pub async fn get_request_example() -> impl IntoResponse {
    let example = r#"{
        "sheet": {
            "width": 2800,
            "length": 2070
        },
        "material": {
            "material_type": "ДСП",
            "thickness": 18.0
        },
        "details": [
            {
                "id": 1,
                "name": "Дверца",
                "width": 500,
                "length": 700,
                "quantity": 2,
                "angle": 0
            },
            {
                "id": 2,
                "name": "Полка",
                "width": 800,
                "length": 300,
                "quantity": 3,
                "angle": 0
            },
            {
                "id": 3,
                "name": "Стенка",
                "width": 600,
                "length": 1200,
                "quantity": 2,
                "angle": 0
            }
        ],
        "layout": {
            "method": "optimal",
            "gap": 5,
            "blade_width": 3,
            "margin": 15,
            "starting_corner": "top-left"
        },
        "edges": [
            {
                "edge_type": "ПВХ",
                "thickness": 0.5
            },
            {
                "edge_type": "Акрил",
                "thickness": 1.0
            }
        ]
    }"#;

    (StatusCode::OK, Json(serde_json::from_str::<Value>(example).unwrap()))
}

// Обработчик для получения примера JSON-структуры ответа
pub async fn get_response_example() -> impl IntoResponse {
    let example = r#"{
        "sheet": {
            "width": 2800,
            "length": 2070,
            "padding": 15,
            "viewBox": {
                "width": 2830,
                "height": 2100
            }
        },
        "layout": {
            "method": "optimal",
            "gap": 5,
            "blade_width": 3,
            "margin": 15,
            "starting_corner": "top-left"
        },
        "details": [
            {
                "id": 3,
                "name": "Стенка",
                "width": 600,
                "length": 1200,
                "angle": 0,
                "x": 15,
                "y": 15,
                "textPosition": {
                    "x": 315,
                    "y": 615
                }
            },
            {
                "id": 3,
                "name": "Стенка",
                "width": 600,
                "length": 1200,
                "angle": 0,
                "x": 623,
                "y": 15,
                "textPosition": {
                    "x": 923,
                    "y": 615
                }
            },
            {
                "id": 1,
                "name": "Дверца",
                "width": 500,
                "length": 700,
                "angle": 0,
                "x": 1231,
                "y": 15,
                "textPosition": {
                    "x": 1481,
                    "y": 365
                }
            },
            {
                "id": 1,
                "name": "Дверца",
                "width": 500,
                "length": 700,
                "angle": 0,
                "x": 1739,
                "y": 15,
                "textPosition": {
                    "x": 1989,
                    "y": 365
                }
            },
            {
                "id": 2,
                "name": "Полка",
                "width": 800,
                "length": 300,
                "angle": 0,
                "x": 15,
                "y": 1223,
                "textPosition": {
                    "x": 415,
                    "y": 1373
                }
            },
            {
                "id": 2,
                "name": "Полка",
                "width": 800,
                "length": 300,
                "angle": 0,
                "x": 823,
                "y": 1223,
                "textPosition": {
                    "x": 1223,
                    "y": 1373
                }
            },
            {
                "id": 2,
                "name": "Полка",
                "width": 800,
                "length": 300,
                "angle": 0,
                "x": 1631,
                "y": 1223,
                "textPosition": {
                    "x": 2031,
                    "y": 1373
                }
            }
        ],
        "statistics": {
            "sheet_area": 5796000,
            "used_area": 4080000,
            "waste_area": 1716000,
            "cut_length": 15400,
            "edge_length": 15400,
            "detail_count": 7,
            "efficiency": 70.39
        }
    }"#;

    (StatusCode::OK, Json(serde_json::from_str::<Value>(example).unwrap()))
}