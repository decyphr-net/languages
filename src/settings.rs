use config::{
    Config, Environment as ConfigEnvironment, File
};
use serde::Deserialize;
use sqlx::{
    postgres::{
        PgConnectOptions,
        PgSslMode,
    }
};
use std::env::{
    current_dir, var
};


#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub debug: bool,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub protocol: String,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn  connect_to_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        let options = PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
            .database(&self.database_name);
        options
    }
}

pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production"
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(
                format!(
                    "{} is not a supported environment. 
                    Use either `development` or `production`", 
                    other
                )
            )
        }
    }
}

pub fn get_settings() -> Result<Settings, config::ConfigError> {
    let base_path = current_dir().expect(
        "Failed to determine the current directory"
    );
    let settings_directory = base_path.join("settings");

    let environment: Environment = var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings = Config::builder()
        .add_source(File::from(settings_directory.join("base.yaml")))
        .add_source(File::from(
            settings_directory.join(environment_filename),
        ))
        .add_source(
            ConfigEnvironment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;
    
    settings.try_deserialize::<Settings>()
}
