#![allow(dead_code)]

use rusql_alchemy::Error;
use rusql_alchemy::prelude::*;
use tokio::sync::OnceCell;

use models::{AppSettings, Favorite, YoutubeTrack};

use super::utils::get_config_path;

pub mod models;

static CONN: OnceCell<Connection> = OnceCell::const_new();

async fn conn() -> &'static Connection {
    CONN.get_or_init(|| async move {
        let path = get_config_path().unwrap();
        if let Some(parent) = std::path::Path::new(&path).parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        let database = Database::new_local(&path).await.unwrap();
        database.up().await.unwrap();
        database.conn
    })
    .await
}

pub async fn add_to_favorite(track: YoutubeTrack) -> Result<(), Error> {
    let conn = conn().await;

    if get_favorite_by(&track.id, conn).await?.is_some() {
        return Ok(());
    }

    let youtube_track = YoutubeTrack::get(kwargs!(id = track.id), conn).await?;
    if youtube_track.is_none() {
        track.save(conn).await?;
    }
    Favorite::create(kwargs!(youtube_track_id = track.id), conn).await?;

    Ok(())
}

pub async fn remove_from_favorite(youtube_track_id: &str) -> Result<(), Error> {
    let conn = conn().await;
    if let Some(favorite) = get_favorite_by(youtube_track_id, conn).await? {
        favorite.delete(conn).await?;
    }
    Ok(())
}

pub async fn is_favorite(youtube_track_id: &str) -> Result<bool, Error> {
    let conn = conn().await;
    Ok(get_favorite_by(youtube_track_id, conn).await?.is_some())
}

async fn get_favorite_by(
    youtube_track_id: &str,
    conn: &Connection,
) -> Result<Option<Favorite>, Error> {
    Ok(Favorite::get(kwargs!(youtube_track_id = youtube_track_id), conn).await?)
}

pub async fn get_all_favorites() -> Result<Vec<YoutubeTrack>, rusql_alchemy::Error> {
    let conn = conn().await;
    let results: Vec<YoutubeTrack> = select!(YoutubeTrack, Favorite)
        .inner_join::<YoutubeTrack, Favorite>(kwargs!(YoutubeTrack.id == Favorite.youtube_track_id))
        .fetch_all(conn)
        .await?;

    Ok(results)
}

pub async fn save_token(token: &str) -> Result<(), Error> {
    let conn = conn().await;
    let mut settings = get_settings().await?;
    settings.youtube_token = token.to_string();
    settings.update(conn).await?;
    Ok(())
}

pub async fn get_settings() -> Result<AppSettings, Error> {
    let conn = conn().await;
    let get_app_setting = || async { AppSettings::get(kwargs!(id = 0), conn).await };
    if let Some(app_setting) = get_app_setting().await? {
        return Ok(app_setting);
    }
    AppSettings::default().save(conn).await?;
    Ok(get_app_setting().await?.unwrap())
}
