#![allow(dead_code)]

use rusql_alchemy::Error;
use rusql_alchemy::prelude::*;
use tokio::sync::OnceCell;

use models::{AppSettings, Favorite, History, Playlist, Track, TrackPlaylist};

use crate::core::platform::get_config_path;

pub mod models;

async fn conn() -> Result<&'static Connection, Error> {
    static CONN: OnceCell<Connection> = OnceCell::const_new();
    CONN.get_or_try_init(|| async move {
        let path = get_config_path()?;
        let database = Database::new_local(path.to_str().unwrap()).await?;
        database.up().await?;
        Ok(database.conn)
    })
    .await
}

pub async fn get_or_create_track(track: &Track, conn: &Connection) -> Result<Track, Error> {
    if let Some(track) = Track::get(kwargs!(track_id = track.track_id), conn).await? {
        return Ok(track);
    }
    track.save(conn).await
}

pub async fn add_to_favorite(track: Track) -> Result<Track, Error> {
    let conn = conn().await?;
    let track_id = &track.track_id;

    if !get_favorite_by(track_id, conn).await?.is_some() {
        let track = get_or_create_track(&track, conn).await?;
        Favorite::create(kwargs!(favorite_fk_track_id = track.track_id), conn).await?;
    }
    Ok(track)
}

pub async fn remove_from_favorite(track_id: &str) -> Result<(), Error> {
    let conn = conn().await?;
    if let Some(favorite) = get_favorite_by(track_id, conn).await? {
        favorite.delete(conn).await?;
    }
    Ok(())
}

pub async fn is_favorite(track_id: &str) -> Result<bool, Error> {
    let conn = conn().await?;
    Ok(get_favorite_by(track_id, conn).await?.is_some())
}

async fn get_favorite_by(track_id: &str, conn: &Connection) -> Result<Option<Favorite>, Error> {
    Favorite::get(kwargs!(favorite_fk_track_id = track_id), conn).await
}

pub async fn get_all_favorites() -> Result<Vec<Track>, Error> {
    let conn = conn().await?;
    let results: Vec<Track> = select!(Track, Favorite)
        .inner_join::<Track, Favorite>(kwargs!(Track.track_id == Favorite.favorite_fk_track_id))
        .fetch_all(conn)
        .await?;

    Ok(results)
}

pub async fn record_play(track: Track) -> Result<(), Error> {
    let conn = conn().await?;
    let track = get_or_create_track(&track, conn).await?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);

    match History::get(kwargs!(history_fk_track_id = track.track_id), conn).await? {
        Some(mut history) => {
            history.play_count += 1;
            history.last_played_at = now;
            history.update(conn).await?;
        }
        None => {
            History::create(
                kwargs!(
                    history_fk_track_id = track.track_id,
                    play_count = 1,
                    last_played_at = now
                ),
                conn,
            )
            .await?;
        }
    }

    Ok(())
}

pub async fn get_all_history() -> Result<Vec<History>, Error> {
    let conn = conn().await?;
    History::all(conn).await
}

pub async fn get_track_by_id(track_id: &str) -> Result<Option<Track>, Error> {
    let conn = conn().await?;
    Track::get(kwargs!(track_id = track_id), conn).await
}

pub async fn save_token(token: &str) -> Result<(), Error> {
    let conn = conn().await?;
    let mut settings = get_settings().await?;
    settings.youtube_token = Some(token.to_string());
    settings.update(conn).await?;
    Ok(())
}

pub async fn save_theme(theme: &str) -> Result<(), Error> {
    let conn = conn().await?;
    let mut settings = get_settings().await?;
    settings.theme = theme.to_string();
    settings.update(conn).await?;
    Ok(())
}

pub async fn get_settings() -> Result<AppSettings, Error> {
    let conn = conn().await?;
    if let Some(app_setting) = AppSettings::get(kwargs!(id = 0), conn).await? {
        return Ok(app_setting);
    }
    let app_setting = AppSettings::default().save(conn).await?;
    Ok(app_setting)
}

pub async fn create_playlist(name: &str) -> Result<Playlist, Error> {
    let conn = conn().await?;
    Playlist::create(kwargs!(name = name), conn).await
}

pub async fn get_playlist_by_id(playlist_id: i32) -> Result<Option<Playlist>, Error> {
    let conn = conn().await?;
    Playlist::get(kwargs!(playlist_id = playlist_id), conn).await
}

pub async fn get_all_playlists() -> Result<Vec<Playlist>, Error> {
    let conn = conn().await?;
    Playlist::all(conn).await
}

pub async fn list_track_playlist(playlist_id: i32) -> Result<Vec<Track>, Error> {
    let conn = conn().await?;

    let results: Vec<Track> = select!(Track, TrackPlaylist)
        .inner_join::<Track, TrackPlaylist>(kwargs!(
            Track.track_id == TrackPlaylist.track_playlist_fk_track_id
        ))
        .r#where(kwargs!(
            TrackPlaylist.track_playlist_fk_playlist_id == playlist_id
        ))
        .fetch_all(conn)
        .await?;

    Ok(results)
}

pub async fn add_to_playlist(track: Track, playlist_id: i32) -> Result<Track, Error> {
    let conn = conn().await?;
    let track = get_or_create_track(&track, conn).await?;

    TrackPlaylist::create(
        kwargs!(
            track_playlist_fk_track_id = track.track_id,
            track_playlist_fk_playlist_id = playlist_id
        ),
        conn,
    )
    .await?;

    Ok(track)
}

pub async fn delete_playlist(playlist_id: i32) -> Result<(), Error> {
    let conn = conn().await?;
    let links: Vec<TrackPlaylist> =
        TrackPlaylist::filter(kwargs!(track_playlist_fk_playlist_id == playlist_id), conn).await?;
    for link in links {
        link.delete(conn).await?;
    }
    if let Some(playlist) = Playlist::get(kwargs!(playlist_id = playlist_id), conn).await? {
        playlist.delete(conn).await?;
    }
    Ok(())
}

pub async fn remove_from_playlist(playlist_id: i32, track_id: &str) -> Result<(), Error> {
    let conn = conn().await?;
    let links: Vec<TrackPlaylist> = TrackPlaylist::filter(
        kwargs!(track_playlist_fk_playlist_id == playlist_id)
            .and(kwargs!(track_playlist_fk_track_id == track_id)),
        conn,
    )
    .await?;
    for link in links {
        link.delete(conn).await?;
    }
    Ok(())
}
