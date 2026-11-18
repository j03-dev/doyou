use rusql_alchemy::prelude::*;
use yt::data_api::types::Item;

use super::db;
use super::db::models::{Favorite, Track};
use super::ensure_track;
use super::identity;

pub async fn all() -> Result<Vec<Track>, String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let tracks: Vec<Track> = select!(Track, Favorite)
        .inner_join::<Track, Favorite>(kwargs!(Track.id == Favorite.track_id))
        .fetch_all(conn)
        .await
        .map_err(|err| err.to_string())?;
    Ok(identity::dedupe(tracks))
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

pub async fn remove(item: &Item) -> Result<(), String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let favorites: Vec<Favorite> = Favorite::all(conn).await.map_err(|err| err.to_string())?;

    for favorite in favorites {
        let track = Track::get(kwargs!(id = favorite.track_id.clone()), conn)
            .await
            .map_err(|err| err.to_string())?;
        if track.is_some_and(|track| identity::same_media(&track, item)) {
            favorite.delete(conn).await.map_err(|err| err.to_string())?;
        }
    }
    Ok(())
}
