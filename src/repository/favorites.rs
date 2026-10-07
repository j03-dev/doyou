use super::db;
use super::db::models::YoutubeTrack;

pub async fn all() -> Result<Vec<YoutubeTrack>, String> {
    db::get_all_favorites().await.map_err(|err| err.to_string())
}

pub async fn add(track: YoutubeTrack) -> Result<YoutubeTrack, String> {
    db::add_to_favorite(track)
        .await
        .map_err(|err| err.to_string())
}

pub async fn remove(youtube_track_id: &str) -> Result<(), String> {
    db::remove_from_favorite(youtube_track_id)
        .await
        .map_err(|err| err.to_string())
}
