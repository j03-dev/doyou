use rusql_alchemy::prelude::*;
use yt::data_api::types::Item;

use super::db;
use super::db::models::{History, Track};
use super::ensure_track;

pub async fn record(item: &Item) -> Result<(), String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let track = ensure_track(item).await?;
    let now = now();

    match History::get(kwargs!(track_id = track.id.clone()), conn)
        .await
        .map_err(|err| err.to_string())?
    {
        Some(mut history) => {
            history.play_count += 1;
            history.last_played_at = now;
            history.update(conn).await.map_err(|err| err.to_string())?;
        }
        None => {
            History::create(
                kwargs!(
                    track_id = track.id.clone(),
                    play_count = 1,
                    last_played_at = now
                ),
                conn,
            )
            .await
            .map_err(|err| err.to_string())?;
        }
    }

    Ok(())
}

pub async fn recently_played(limit: usize) -> Result<Vec<Track>, String> {
    let mut history = all_history().await?;
    history.sort_by(|a, b| b.last_played_at.total_cmp(&a.last_played_at));
    load_tracks(history.into_iter().take(limit).map(|h| h.track_id)).await
}

pub async fn most_played(limit: usize) -> Result<Vec<Track>, String> {
    let mut history = all_history().await?;
    history.sort_by(|a, b| {
        b.play_count
            .cmp(&a.play_count)
            .then_with(|| b.last_played_at.total_cmp(&a.last_played_at))
    });
    load_tracks(history.into_iter().take(limit).map(|h| h.track_id)).await
}

async fn all_history() -> Result<Vec<History>, String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    History::all(conn).await.map_err(|err| err.to_string())
}

async fn load_tracks(ids: impl Iterator<Item = String>) -> Result<Vec<Track>, String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let mut tracks = Vec::new();
    for track_id in ids {
        if let Some(track) = Track::get(kwargs!(id = track_id), conn)
            .await
            .map_err(|err| err.to_string())?
        {
            tracks.push(track);
        }
    }
    Ok(tracks)
}

fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs_f64())
        .unwrap_or(0.0)
}
