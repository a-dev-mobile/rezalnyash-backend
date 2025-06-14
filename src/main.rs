mod features;
mod shared;

use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tracing::{debug, error, info, warn};

use crate::shared::{
    database::{migrations::run_migrations, service::PostgresService},
    logger, middleware,
    setting::models::{app_config::AppConfig, app_env::AppEnv, app_setting::AppSettings, app_state::AppState},
};

// Дополнительные импорты для JSON и времени
use chrono;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize application settings and logging
    let settings: Arc<AppSettings> = Arc::new(init_app().await);
    info!("🔪 Запуск РезальНяш 🔪");
    info!("Приложение запущено");
    debug!("Это отладочное сообщение");
    warn!("Предупреждение");
    error!("Ошибка: {}", "что-то пошло не так");

    // Connect to databases
    let postgres_service = Arc::new(initialize_database(settings.clone()).await?);

    let server_address = format!("{}:{}", settings.env.server_address, settings.env.server_port)
        .parse()
        .expect("Invalid server address configuration");

    // Create application state with all services and dependencies
    let app_state = Arc::new(AppState::new(settings.clone(), postgres_service).await);

    // Create API router using app_state
    let app_router = create_application_router(app_state);

    // Start HTTP server
    start_http_server(app_router, server_address).await;

    Ok(())
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

    if app_settings.env.is_development() {
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
        // CORS и трейсинг middleware
        .layer(middleware::create_cors())
        .layer(middleware::create_trace())
        // === HEALTH ENDPOINTS ===
        .route(
            "/health",
            get({
                let app_state = Arc::clone(&app_state);
                move || async move { app_state.material_health_handler.get_health().await }
            }),
        )

        // === MATERIALS ENDPOINT - NEW ===
        .route(
            "/api/v1/materials",
            get({
                let app_state = Arc::clone(&app_state);
                move || async move { app_state.material_handler.get_all_materials().await }
            }),
        )

        // === MATERIAL TYPES ENDPOINTS ===
        .route(
            "/api/v1/materials/types",
            get({
                let app_state = Arc::clone(&app_state);
                move || async move { app_state.material_type_handler.get_all_types().await }
            }),
        )
        .route(
            "/api/v1/materials/types",
            post({
                let app_state = Arc::clone(&app_state);
                move |payload| async move { app_state.material_type_handler.create_type(payload).await }
            }),
        )
        .route(
            "/api/v1/materials/types/{id}",
            get({
                let app_state = Arc::clone(&app_state);
                move |path| async move { app_state.material_type_handler.get_type(path).await }
            }),
        )

        // === MATERIAL NAMES ENDPOINTS ===
        .route(
            "/api/v1/materials/names",
            get({
                let app_state = Arc::clone(&app_state);
                move || async move { app_state.material_name_handler.get_all_names().await }
            }),
        )
        .route(
            "/api/v1/materials/names",
            post({
                let app_state = Arc::clone(&app_state);
                move |payload| async move { app_state.material_name_handler.create_name(payload).await }
            }),
        )
        .route(
            "/api/v1/materials/names/{id}",
            get({
                let app_state = Arc::clone(&app_state);
                move |path| async move { app_state.material_name_handler.get_name(path).await }
            }),
        )

        // === WIDTHS ENDPOINTS ===
        .route(
            "/api/v1/materials/widths",
            get({
                let app_state = Arc::clone(&app_state);
                move || async move { app_state.material_width_handler.get_all_widths().await }
            }),
        )
        .route(
            "/api/v1/materials/widths",
            post({
                let app_state = Arc::clone(&app_state);
                move |payload| async move { app_state.material_width_handler.create_width(payload).await }
            }),
        )
        .route(
            "/api/v1/materials/widths/{id}",
            get({
                let app_state = Arc::clone(&app_state);
                move |path| async move { app_state.material_width_handler.get_width(path).await }
            }),
        )

        // === HEIGHTS ENDPOINTS ===
        .route(
            "/api/v1/materials/heights",
            get({
                let app_state = Arc::clone(&app_state);
                move || async move { app_state.material_height_handler.get_all_heights().await }
            }),
        )
        .route(
            "/api/v1/materials/heights",
            post({
                let app_state = Arc::clone(&app_state);
                move |payload| async move { app_state.material_height_handler.create_height(payload).await }
            }),
        )
        .route(
            "/api/v1/materials/heights/{id}",
            get({
                let app_state = Arc::clone(&app_state);
                move |path| async move { app_state.material_height_handler.get_height(path).await }
            }),
        )

        // === THICKNESS ENDPOINTS ===
        .route(
            "/api/v1/materials/thicknesses",
            get({
                let app_state = Arc::clone(&app_state);
                move || async move { app_state.material_thickness_handler.get_all_thicknesses().await }
            }),
        )
        .route(
            "/api/v1/materials/thicknesses",
            post({
                let app_state = Arc::clone(&app_state);
                move |payload| async move { app_state.material_thickness_handler.create_thickness(payload).await }
            }),
        )
        .route(
            "/api/v1/materials/thicknesses/{id}",
            get({
                let app_state = Arc::clone(&app_state);
                move |path| async move { app_state.material_thickness_handler.get_thickness(path).await }
            }),
        )

        // Добавляем состояние приложения как Extension
        .layer(axum::Extension(app_state))
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

async fn initialize_database(settings: Arc<AppSettings>) -> Result<PostgresService, Box<dyn std::error::Error>> {
    info!("Initializing database connections...");

    let postgres_service = PostgresService::new(&settings).await?;

    info!("Running database migrations...");
    run_migrations(postgres_service.connection.pool()).await?;

    Ok(postgres_service)
}
