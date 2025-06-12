mod api;
mod database;
mod logger;
mod middleware;
mod models;
mod setting;
mod utils;

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing::{debug, error, info, warn};

use setting::models::{
    app_config::AppConfig, app_env::AppEnv, app_setting::AppSettings, app_state::AppState,
};

use std::{net::SocketAddr, sync::Arc};

use crate::database::service::PostgresService;

#[tokio::main]
async fn main() {
    // Initialize application settings and logging
    let settings: Arc<AppSettings> = Arc::new(init_app().await);
    info!("🔪 Запуск РезальНяш 🔪");
    info!("Приложение запущено");
    debug!("Это отладочное сообщение");
    warn!("Предупреждение");
    error!("Ошибка: {}", "что-то пошло не так");

    // Connect to databases
    let postgres_service = initialize_database(settings.clone()).await;

    // Создаем состояние приложения
    // let state = app_state::create_state();

    // Настраиваем и запускаем сервер
    // Parse server address from configuration
    let server_address: SocketAddr = format!(
        "{}:{}",
        settings.env.server_address, settings.env.server_port,
    )
    .parse()
    .expect("Invalid server address configuration");

    // Create application state with all services
    let app_state: Arc<AppState> =
        Arc::new(AppState::new(settings.clone(), Arc::new(postgres_service)).await);

    // Create API router
    let app_router = create_application_router(app_state.clone());

    // Start HTTP server
    start_http_server(app_router, server_address).await;
}

async fn init_app() -> AppSettings {
    let environment = AppEnv::new();
    let config = AppConfig::new(&environment.env);
    info!("Инициализация приложения с конфигурацией: {:?}", config);
    let app_settings = AppSettings {
        config,
        env: environment,
    };
    // Setup logging with configured level and format
    logger::init_logger(
        &app_settings.config.logging.level,
        &app_settings.config.logging.format,
        app_settings.env.is_prod(),
    )
    .expect("Failed to initialize logger");

    // Log application startup information
    info!("Starting application...");
    info!("Current environment: {}", app_settings.env.env);

    if app_settings.env.is_local() {
        info!("Running in local development mode");
        debug!("Configuration details: {:#?}", app_settings);
    } else {
        info!("Running in production mode");
    }

    app_settings
}

/// Creates the application router with all API endpoints and middleware
fn create_application_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .layer(middleware::create_cors())
        .route("/api-health", get(api::health_api))
        // .route("/db-health", get(api::health_db))
        .layer(axum::Extension(app_state.clone()))
        .layer(middleware::create_trace())
}

/// Starts the HTTP server on the specified address
async fn start_http_server(app: Router, addr: SocketAddr) {
    info!("Starting HTTP server on {}", addr);

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            error!("Failed to bind to address {}: {}", addr, err);
            panic!("Cannot start server: {}", err);
        }
    };

    info!("Server started successfully, now accepting connections");

    if let Err(err) = axum::serve(listener, app).await {
        error!("Server error: {}", err);
        panic!("Server failed: {}", err);
    }
}

async fn initialize_database(settings: Arc<AppSettings>) -> PostgresService {
    info!("Initializing database connections...");
    match PostgresService::new(&settings).await {
        Ok(service) => service,
        Err(err) => {
            error!("Failed to connect to PostgreSQL: {}", err);
            panic!("Cannot continue without PostgreSQL connection");
        }
    }
}
