use anyhow::Result;
use rusql_alchemy::prelude::*;

use super::db;
use super::db::models::AppSettings;

pub async fn load() -> Result<AppSettings> {
    let conn = db::conn().await?;
    current(conn).await
}

pub async fn save_token(token: &str) -> Result<()> {
    let conn = db::conn().await?;
    let mut settings = current(conn).await?;
    settings.youtube_token = Some(token.to_string());
    Ok(settings.update(conn).await?)
}

pub async fn save_theme(theme: &str) -> Result<()> {
    let conn = db::conn().await?;
    let mut settings = current(conn).await?;
    settings.theme = theme.to_string();
    Ok(settings.update(conn).await?)
}

async fn current(conn: &Connection) -> Result<AppSettings> {
    if let Some(settings) = AppSettings::get(kwargs!(id = 0), conn).await? {
        return Ok(settings);
    }
    Ok(AppSettings::default().save(conn).await?)
}
