use rusql_alchemy::prelude::*;

use super::db;
use super::db::models::AppSettings;

pub async fn load() -> Result<AppSettings, String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    current(conn).await
}

pub async fn save_token(token: &str) -> Result<(), String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let mut settings = current(conn).await?;
    settings.youtube_token = Some(token.to_string());
    settings.update(conn).await.map_err(|err| err.to_string())
}

pub async fn save_theme(theme: &str) -> Result<(), String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let mut settings = current(conn).await?;
    settings.theme = theme.to_string();
    settings.update(conn).await.map_err(|err| err.to_string())
}

async fn current(conn: &Connection) -> Result<AppSettings, String> {
    if let Some(settings) = AppSettings::get(kwargs!(id = 0), conn)
        .await
        .map_err(|err| err.to_string())?
    {
        return Ok(settings);
    }
    AppSettings::default()
        .save(conn)
        .await
        .map_err(|err| err.to_string())
}
