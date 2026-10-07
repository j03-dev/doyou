use crate::core::db;
use crate::core::db::models::AppSettings;

pub async fn load() -> Result<AppSettings, String> {
    db::get_settings().await.map_err(|err| err.to_string())
}

pub async fn save_token(token: &str) -> Result<(), String> {
    db::save_token(token).await.map_err(|err| err.to_string())
}

pub async fn save_theme(theme: &str) -> Result<(), String> {
    db::save_theme(theme).await.map_err(|err| err.to_string())
}
