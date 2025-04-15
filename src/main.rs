mod models;
mod server;
mod handlers;
mod json_generator;
mod app_state;

use log::{debug, error, info, warn};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    println!("🔪 Запуск РезальНяш v0.2.0 🔪");
    
    // Инициализация логгера
    env_logger::init();
    
    info!("Приложение запущено");
    debug!("Это отладочное сообщение");
    warn!("Предупреждение");
    error!("Ошибка: {}", "что-то пошло не так");
    
    // Создаем состояние приложения
    let state = app_state::create_state();
    
    // Настраиваем и запускаем сервер
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("РезальНяш запущен на http://{}", addr);
    println!("Ожидаем запросы на няшный раскрой...");
    println!("Доступные API:");
    println!("  JSON API: POST/GET /api/sheet");
    println!("  Примеры: GET /api/examples/request - пример запроса");
    println!("           GET /api/examples/response - пример ответа");
    
    server::start_server(state, addr).await;
}