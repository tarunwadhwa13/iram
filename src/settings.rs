use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub engine: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SelectedFeature<T> {
    pub supported: Vec<T>,
    pub enabled: Vec<T>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub env: ENV,
    pub database: Database,
    pub alert_sources: SelectedFeature<String>,
    pub auth_backends: SelectedFeature<String>,
    pub notification_destinations: SelectedFeature<String>,
}

const CONFIG_FILE_PATH: &str = "./config/Default.toml";
const CONFIG_FILE_PREFIX: &str = "./config/";

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Dev".into());
        let builder = Config::builder()
            .set_override("env", env.clone())?
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .add_source(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))
            // This makes it so "IRAM_SERVER__PORT overrides server.port
            .add_source(Environment::with_prefix("iram").separator("__"))
            .build()?;

        builder.try_deserialize()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Dev,
    Test,
    Prod,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Dev => write!(f, "Dev"),
            ENV::Test => write!(f, "Test"),
            ENV::Prod => write!(f, "Prod"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Test" => ENV::Test,
            "Prod" => ENV::Prod,
            _ => ENV::Dev,
        }
    }
}
