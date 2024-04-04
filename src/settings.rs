use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

pub enum RunMode {
    Production,
    Development,
}

impl RunMode {
    pub fn get() -> Self {
        let run_mode_str = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        match run_mode_str.as_str() {
            "production" => RunMode::Production,
            _ => RunMode::Development,
        }
    }
}

impl std::fmt::Display for RunMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunMode::Production => write!(f, "production"),
            RunMode::Development => write!(f, "development"),
        }
    }
}

#[derive(Deserialize)]
pub struct SslSettings {
    pub private_key_file: String,
    pub certification_chain_file: String,
    pub ca_file: String,
}

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
    pub ssl: Option<SslSettings>,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl Settings {
    pub fn new(run_mode: &RunMode) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("settings/default").required(false))
            .add_source(File::with_name(&format!("settings/{run_mode}")))
            .add_source(Environment::with_prefix("shopp").separator("_"))
            .build()?;

        s.try_deserialize()
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}
