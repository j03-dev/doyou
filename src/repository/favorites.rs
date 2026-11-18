use rusql_alchemy::prelude::*;
use yt::data_api::types::Item;

use super::db;
use super::db::models::{Favorite, Track};
use super::ensure_track;

pub async fn all() -> Result<Vec<Track>, String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let tracks: Vec<Track> = select!(Track, Favorite)
        .inner_join::<Track, Favorite>(kwargs!(Track.id == Favorite.track_id))
        .fetch_all(conn)
        .await
        .map_err(|err| err.to_string())?;
    Ok(tracks)
}

pub async fn add(item: &Item) -> Result<Track, String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let track = ensure_track(item).await?;

    if Favorite::get(kwargs!(track_id = track.id.clone()), conn)
        .await
        .map_err(|err| err.to_string())?
        .is_none()
    {
        Favorite::create(kwargs!(track_id = track.id.clone()), conn)
            .await
            .map_err(|err| err.to_string())?;
    }
    Ok(track)
}

pub async fn remove(track_id: &str) -> Result<(), String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    if let Some(favorite) = Favorite::get(kwargs!(track_id = track_id), conn)
        .await
        .map_err(|err| err.to_string())?
    {
        favorite.delete(conn).await.map_err(|err| err.to_string())?;
    }
    Ok(())
}
