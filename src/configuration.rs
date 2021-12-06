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

/// Tries to read the `configuration.yml` file in the current
/// working directory and returns the configuration as a [`AppSettings`].
/// It reads also from the environment, and thus it can override settings
/// specified in the YAML file.
/// For example `CONDUIT_DATABASE__PASSWORD=password` would set the
/// `AppSettings.database.password` field.
pub fn read_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("configuration"))?
        .merge(config::Environment::with_prefix("CONDUIT").separator("__"))?;

    settings.try_into()
}
