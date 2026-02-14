use std::{fs, path};

use serde::{Deserialize, Serialize};

use crate::core::error::Error;
use crate::core::utils::get_config_path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub youtube_token: String,
}

impl AppConfig {
    pub fn new(youtube_token: String) -> Self {
        Self { youtube_token }
    }

    pub fn load() -> Result<Option<Self>, Error> {
        let path = get_config_path()?;
        if let Ok(content) = fs::read_to_string(&path) {
            let config = serde_json::from_str(&content)?;
            return Ok(Some(config));
        }
        Ok(None)
    }

    pub fn save(&self) -> Result<(), Error> {
        let path = get_config_path()?;
        if let Some(parent) = path::Path::new(&path).parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string(self)?;
        fs::write(&path, json)?;
        Ok(())
    }
}
