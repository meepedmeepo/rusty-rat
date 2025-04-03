use std::env;

use anyhow::{Context, Error, Result};
use config::{Config, ConfigError};

///Loads settings from settings.toml -> mainly used for selecting port to run server on
pub fn init() -> Result<Config> {
    common::current_dir()?;

    let settings = Config::builder()
        .add_source(config::File::with_name("src/configuration/settings"))
        .build()
        .context("Failed to load config file!")?;

    Ok(settings)
}
