use config::Config;
use secrecy::{ExposeSecret, Secret};

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug, serde::Deserialize)]
pub struct AppSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
    pub port: u16,
    pub name: String,
}

impl DatabaseSettings {
    pub fn connect_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        ))
    }

    pub fn connect_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
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
    let base_path =
        std::env::current_dir().expect("Failed to determine the current directory");
    let config_path = base_path.join("config");

    let app_env: AppEnv = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "dev".into())
        .try_into()
        .expect("Failed to parse APP_ENV");

    Config::builder()
        .add_source(config::File::from(config_path.join("config")))
        .add_source(config::File::from(config_path.join(app_env.as_str())))
        .add_source(
            config::Environment::default()
                .try_parsing(true)
                .separator("_"),
        )
        .build()?
        .try_deserialize()
}
