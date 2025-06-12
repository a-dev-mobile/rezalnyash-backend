use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

use crate::error::{ApiResult, AppError};

pub async fn health_api1() -> StatusCode {
    // info!("Handling test request");
    StatusCode::OK
}

pub async fn health_api() -> ApiResult<Json<Value>> {
    let result = json!({
            "status": "healthy",
    });

    Ok(Json(result))

    // Err(AppError::InternalError { message:  "111test error".to_string(),
    // })
}
