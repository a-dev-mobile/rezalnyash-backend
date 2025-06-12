mod api;
mod database;
mod error;
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

use crate::database::{migrations::run_migrations, service::PostgresService};

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

    let server_address = format!(
        "{}:{}",
        settings.env.server_address, settings.env.server_port
    )
    .parse()
    .expect("Invalid server address configuration");

    // Create application state with all services
    let app_state = Arc::new(AppState::new(settings.clone(), postgres_service).await);

    // Create API router
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
        .layer(middleware::create_cors())
        .route("/api-health", get(api::v1::health_api))
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

async fn initialize_database(
    settings: Arc<AppSettings>,
) -> Result<PostgresService, Box<dyn std::error::Error>> {
    info!("Initializing database connections...");

    let postgres_service = PostgresService::new(&settings).await?;

    info!("Running database migrations...");
    run_migrations(postgres_service.connection.pool()).await?;

    Ok(postgres_service)
}
