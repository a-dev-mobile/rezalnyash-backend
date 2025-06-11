use std::sync::Arc;

use crate::setting::models::app_setting::AppSettings;

pub struct AppState {
    pub settings: Arc<AppSettings>,
}

impl AppState {
    pub async fn new(settings: Arc<AppSettings>) -> Self {
        Self { settings }
    }
}
