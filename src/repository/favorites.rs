use anyhow::Result;
use rusql_alchemy::prelude::*;
use yt::data_api::types::Item;

use super::db;
use super::db::models::{Favorite, Track};
use super::ensure_track;
use super::identity;

pub async fn all() -> Result<Vec<Track>> {
    let conn = db::conn().await?;
    let tracks: Vec<Track> = select!(Track, Favorite)
        .inner_join::<Track, Favorite>(kwargs!(Track.id == Favorite.track_id))
        .fetch_all(conn)
        .await?;
    Ok(identity::dedupe(tracks))
}

pub async fn add(item: &Item) -> Result<Track> {
    let conn = db::conn().await?;
    let track = ensure_track(item).await?;

    if Favorite::get(kwargs!(track_id = track.id.clone()), conn)
        .await?
        .is_none()
    {
        Favorite::create(kwargs!(track_id = track.id.clone()), conn).await?;
    }
    Ok(track)
}

pub async fn remove(item: &Item) -> Result<()> {
    let conn = db::conn().await?;
    let favorites: Vec<Favorite> = Favorite::all(conn).await?;

    for favorite in favorites {
        let track = Track::get(kwargs!(id = favorite.track_id.clone()), conn).await?;
        if track.is_some_and(|track| identity::same_media(&track, item)) {
            favorite.delete(conn).await?;
        }
    }
    Ok(())
}
