use std::path::PathBuf;

#[cfg(target_os = "android")]
pub use android::*;

#[cfg(not(target_os = "android"))]
pub use desktop::*;

use crate::core::error::Error;

#[cfg(not(target_os = "android"))]
mod desktop;

#[cfg(not(target_os = "android"))]
mod local_server;

#[cfg(target_os = "android")]
mod android;

pub fn get_config_path() -> Result<PathBuf, Error> {
    let config_dir = get_config_dir()?;
    std::fs::create_dir_all(&config_dir)?;
    Ok(config_dir.join("config.db"))
}

fn app_subdir(name: &str) -> Result<PathBuf, Error> {
    let directory = get_config_dir()?.join(name);
    std::fs::create_dir_all(&directory)?;
    Ok(directory)
}

/// App-private folder for audio downloaded from YouTube.
pub fn downloads_dir() -> Result<PathBuf, Error> {
    app_subdir("downloads")
}

/// App-private folder for artwork extracted from local audio files.
pub fn covers_dir() -> Result<PathBuf, Error> {
    app_subdir("covers")
}
