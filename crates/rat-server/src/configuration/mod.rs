use std::{
    env,
    sync::{Arc, LazyLock, Mutex},
};

use anyhow::{Context, Error, Result};
use config::{Config, ConfigError};
use ed25519_dalek::VerifyingKey;

///Loads settings from settings.toml -> mainly used for selecting port to run server on
pub fn init() -> Result<(Arc<Mutex<AppConfig>>)> {
    common::current_dir()?;

    let settings = Config::builder()
        .add_source(config::File::with_name("src/configuration/settings"))
        .build()
        .context("Failed to load config file!")?;

    let config = Arc::new(Mutex::new(AppConfig::new(settings)?));
    Ok(config)
}

///TODO rework this so all values are presaved so don't need to fuck about with casting to types from the Config struct
#[derive(Debug, Clone)]
pub struct AppState {
    pub config: AppConfig,
}

impl AppState {
    ///TODO add validation!
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub client_identity_public_key: VerifyingKey,
}

// impl AppConfig {
//     const fn empty() -> Self {
//         AppConfig {
//             port: 0,
//             client_identity_public_key: None,
//         }
//     }
// }

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 0,
            client_identity_public_key: VerifyingKey::default(),
        }
    }
}

impl AppConfig {
    pub fn new(config: Config) -> Result<Self, Error> {
        let port = config.get::<u16>("port")?;
        let key_str = config.get::<String>("client_public_identity_key")?;

        let client_identity_public_key = VerifyingKey::try_from(key_str.as_bytes())?;

        Ok(Self {
            port,
            client_identity_public_key,
        })
    }
}
