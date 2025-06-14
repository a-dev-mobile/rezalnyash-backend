use axum::response::Response;

#[async_trait::async_trait]
pub trait HealthHandler: Send + Sync {
    /// Проверить общее здоровье приложения
    async fn check_application_health(&self) -> Response;
    
    /// Проверить здоровье базы данных
    async fn check_database_health(&self) -> Response;
}