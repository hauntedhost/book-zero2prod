use config::Config;
use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::PgConnectOptions;
use sqlx::postgres::PgSslMode;
use sqlx::ConnectOptions;

use crate::domain::SubscriberEmail;

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    pub database: DatabaseSettings,
    pub email_client: EmailClientSettings,
}

#[derive(Debug, serde::Deserialize)]
pub struct AppSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub name: String,
    pub require_ssl: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct EmailClientSettings {
    pub base_url: String,
    pub auth_token: Secret<String>,
    pub sender_email: String,
}

impl EmailClientSettings {
    pub fn sender(&self) -> Result<SubscriberEmail, String> {
        SubscriberEmail::parse(self.sender_email.clone())
    }
}

impl DatabaseSettings {
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db()
            .database(&self.name)
            .log_statements(tracing::log::LevelFilter::Trace)
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
}

pub enum AppEnv {
    Dev,
    Prod,
}

impl AppEnv {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppEnv::Dev => "dev",
            AppEnv::Prod => "prod",
        }
    }
}

impl TryFrom<String> for AppEnv {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            other => Err(format!(
                "{} is not a supported environment. Use either `dev` or `prod`.",
                other
            )),
        }
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let config_path = base_path.join("config");

    let app_env: AppEnv = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "dev".into())
        .try_into()
        .expect("Failed to parse APP_ENV");

    Config::builder()
        .add_source(config::File::from(config_path.join("config")))
        .add_source(config::File::from(config_path.join(app_env.as_str())))
        .add_source(config::Environment::default().try_parsing(true).separator("_"))
        .build()?
        .try_deserialize()
}
