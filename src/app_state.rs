use std::sync::{Arc, Mutex};
use crate::models::SheetRequest;

#[derive(Clone)]
pub struct AppState {
    pub sheet_data: Arc<Mutex<Option<SheetRequest>>>,
}

pub fn create_state() -> AppState {
    AppState {
        sheet_data: Arc::new(Mutex::new(None)),
    }
}