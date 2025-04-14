use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::app_state::AppState;
use crate::handlers::{create_sheet, get_sheet};

pub async fn start_server(state: AppState, addr: SocketAddr) {
    // Настраиваем CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Настраиваем маршрутизацию
    let app = Router::new()
        .route("/sheet", post(create_sheet))
        .route("/sheet", get(get_sheet))
        .layer(cors)
        .with_state(state);

    // Запускаем сервер
    start_http_server(app, addr).await;
}

async fn start_http_server(app: Router, addr: SocketAddr) {
    println!("Starting HTTP server on {}", addr);
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Failed to bind to address {}: {}", addr, err);
            panic!("Cannot start server: {}", err);
        }
    };

    println!("Server started successfully, now accepting connections");

    if let Err(err) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", err);
        panic!("Server failed: {}", err);
    }
}