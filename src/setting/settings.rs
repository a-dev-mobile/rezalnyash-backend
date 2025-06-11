#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub optimization: OptimizationConfig,
    pub pdf: PdfConfig,
    pub logging: LoggingConfig,
}

impl Settings {
    pub fn load() -> Result<Self, config::ConfigError> {
        let env = env::var("APP_ENV");
        
        let builder = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::File::with_name(&format!("config/{}", env)).required(false))
            .add_source(config::File::with_name("config/local").required(false))
            .add_source(config::Environment::with_prefix("OPTIMIZER").separator("__"));
            
        builder.build()?.try_deserialize()
    }
}