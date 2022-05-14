//! This module is dealing with configuration loading during the startup
//! of the application. The configuration can be found in a `configuration.yml`
//! file in the current working dir, and values can be overriden with
//! environment variables.

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    ConnectOptions,
};

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    pub database: DatabaseSettings,
}

#[derive(Clone, Deserialize)]
pub struct AppSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub jwt_secret: String,
}

#[derive(Clone, Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub ssl: bool,
}

impl DatabaseSettings {
    /// Returns [`PgConnectOptions`] for a Postgres instance without
    /// specifying the database to work on (default is the "master" DB).
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    /// Returns a [`PgConnectOptions`] for a Postgres instance on the
    /// database specified in the settings.
    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);
        options.log_statements(log::LevelFilter::Trace);
        options
    }
}

/// Retrieves the configurations in the `configuration` directory.
/// The configuration to choose (either `local` or `production`) is read from
/// the `APP_ENVIRONMENT` environment variable.
/// Returns the configuration as a [`AppSettings`].
/// It reads also from the environment, and thus it can override settings
/// specified in the YAML file.
/// For example `CONDUIT__DATABASE__PASSWORD=password` would set the
/// `AppSettings.database.password` field.
pub fn read_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory.");
    let config_dir = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let settings = config::Config::builder()
        .add_source(config::File::from(config_dir.join("base"))) // Base configuration
        .add_source(config::File::from(config_dir.join(environment.as_str()))) // Environment-specific configuration
        .add_source(config::Environment::with_prefix("CONDUIT").separator("__"))
        .build()?;

    settings.try_deserialize()
}

/// Defines the environment (thus the corresponding configuration file) in which
/// the app runs.
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}
