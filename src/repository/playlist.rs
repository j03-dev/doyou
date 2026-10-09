use super::db;
use super::db::models::{Playlist, Track};

pub async fn create(name: &str) -> Result<Playlist, String> {
    db::create_playlist(name)
        .await
        .map_err(|err| err.to_string())
}

pub async fn add(track: Track, playlist_id: i32) -> Result<Track, String> {
    db::add_to_playlist(track, playlist_id)
        .await
        .map_err(|err| err.to_string())
}
