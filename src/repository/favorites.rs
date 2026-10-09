use super::db;
use super::db::models::Track;

pub async fn all() -> Result<Vec<Track>, String> {
    db::get_all_favorites().await.map_err(|err| err.to_string())
}

pub async fn add(track: Track) -> Result<Track, String> {
    db::add_to_favorite(track)
        .await
        .map_err(|err| err.to_string())
}

pub async fn remove(track_id: &str) -> Result<(), String> {
    db::remove_from_favorite(track_id)
        .await
        .map_err(|err| err.to_string())
}
