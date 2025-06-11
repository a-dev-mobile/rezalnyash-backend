
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub logging: LogConfig,


}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize)]
pub struct ClickhouseConfig {
    pub timeout: u64,
}

