use std::path::PathBuf;

#[cfg(target_os = "android")]
pub use android::*;

#[cfg(not(target_os = "android"))]
pub use desktop::*;

use crate::core::error::Error;

#[cfg(not(target_os = "android"))]
mod desktop;

#[cfg(target_os = "android")]
mod android;

pub fn get_config_path() -> Result<PathBuf, Error> {
    let config_dir = get_config_dir()?;
    std::fs::create_dir_all(&config_dir)?;
    Ok(config_dir.join("config.db"))
}
