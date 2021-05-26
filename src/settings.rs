use std::{ffi::OsString, net::TcpListener};

use config::{Config, ConfigError, FileFormat};

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize)]
pub struct HttpSettings {
    pub host: String,
    pub port: u16,
}

impl HttpSettings {
    pub fn tcp_listener(&self) -> std::io::Result<TcpListener> {
        let address = format!("{}:{}", self.host, self.port);
        TcpListener::bind(address)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize)]
pub struct EmailSettings {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub mailhog_host: String,
    pub mailhog_port: u16,
    pub from: String,
    pub recipients: Vec<String>,
    pub backup_dir: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize)]
pub struct LogSettings {
    pub directive: String,
    pub log_dir: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize)]
pub struct Settings {
    pub http: HttpSettings,
    pub email: EmailSettings,
    pub log: LogSettings,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnvironmentError {
    NotUnicode(OsString),
}

const PREFIX: &str = "CONTACT_API";
const SETTINGS_FILE_NAME: &str = "settings";

impl Settings {
    pub fn environment() -> Result<String, EnvironmentError> {
        use std::env::{var, VarError};

        match var(format!("{}_ENVIRONMENT", PREFIX)) {
            Ok(env) => Ok(env.trim().to_owned()),
            Err(VarError::NotPresent) => Ok(String::from("")),
            Err(VarError::NotUnicode(s)) => Err(EnvironmentError::NotUnicode(s)),
        }
    }

    pub fn from_env(environemnt: &str) -> Result<Self, ConfigError> {
        let mut config = Config::default();

        let base_settings = format!("{}.yaml", SETTINGS_FILE_NAME);
        config.merge(config::File::new(&base_settings, FileFormat::Yaml).required(false))?;

        if !environemnt.is_empty() {
            let env_settings = format!("{}.{}.yaml", SETTINGS_FILE_NAME, environemnt);
            config.merge(config::File::new(&env_settings, FileFormat::Yaml).required(false))?;
        }

        config.merge(config::Environment::with_prefix(PREFIX).separator("_"))?;

        config.try_into()
    }

    pub fn new() -> Result<Self, ConfigError> {
        let env = Settings::environment().expect("Failed to read enviromnent from env variable.");
        Settings::from_env(&env)
    }
}
