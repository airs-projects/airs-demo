use std::{fs, path::PathBuf};

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
