use std::{
    fs,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use airs_config::{ConfigError, ConfigHandler};
use serde::{Deserialize, Serialize};

use crate::assets::Assets;

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigData {
    pub window: WindowConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub max_frame_rate: u32,
}

pub struct Config {
    inner: airs_config::Config<ConfigData>,
}

impl Config {
    #[tracing::instrument(skip_all)]
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            inner: airs_config::Config::new()?,
        })
    }
}

impl Deref for Config {
    type Target = ConfigData;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Config {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ConfigHandler for ConfigData {
    fn default_config() -> airs_config::Result<String> {
        let config = Assets::get("config.default.toml")
            .expect("default config must be embedded in the application");
        Ok(String::from_utf8(config.data.into_owned())?)
    }

    fn read_config() -> airs_config::Result<Option<String>> {
        let path = PathBuf::from("config.toml");
        match fs::read_to_string(&path) {
            Ok(config) => Ok(Some(config)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(source) => Err(ConfigError::Read { path, source }),
        }
    }

    fn write_config(config: &str) -> airs_config::Result<()> {
        let path = PathBuf::from("config.toml");
        fs::write(&path, config).map_err(|source| ConfigError::Write { path, source })
    }
}
