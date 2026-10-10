use super::db;
use super::db::models::Track;

pub async fn record(track: Track) -> Result<(), String> {
    db::record_play(track)
        .await
        .map_err(|err| err.to_string())
}

pub async fn recently_played(limit: usize) -> Result<Vec<Track>, String> {
    let mut history = db::get_all_history()
        .await
        .map_err(|err| err.to_string())?;
    history.sort_by(|a, b| b.last_played_at.total_cmp(&a.last_played_at));
    load_tracks(history.into_iter().take(limit).map(|h| h.history_fk_track_id)).await
}

pub async fn most_played(limit: usize) -> Result<Vec<Track>, String> {
    let mut history = db::get_all_history()
        .await
        .map_err(|err| err.to_string())?;
    history.sort_by(|a, b| b.play_count.cmp(&a.play_count));
    load_tracks(history.into_iter().take(limit).map(|h| h.history_fk_track_id)).await
}

async fn load_tracks(ids: impl Iterator<Item = String>) -> Result<Vec<Track>, String> {
    let mut tracks = Vec::new();
    for track_id in ids {
        if let Some(track) = db::get_track_by_id(&track_id)
            .await
            .map_err(|err| err.to_string())?
        {
            tracks.push(track);
        }
    }
    Ok(tracks)
}
