use super::db;
use super::db::models::{Playlist, Track};

#[derive(Debug, Clone, PartialEq)]
pub struct PlaylistWithTracks {
    pub playlist: Playlist,
    pub tracks: Vec<Track>,
}

impl PlaylistWithTracks {
    pub fn thumbnails(&self) -> Vec<String> {
        self.tracks
            .iter()
            .take(4)
            .map(|track| track.thumbnail_url.clone())
            .collect()
    }
}

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

pub async fn list() -> Result<Vec<PlaylistWithTracks>, String> {
    let playlists = db::get_all_playlists()
        .await
        .map_err(|err| err.to_string())?;

    let mut result = Vec::with_capacity(playlists.len());
    for playlist in playlists {
        let tracks = db::list_track_playlist(playlist.playlist_id)
            .await
            .map_err(|err| err.to_string())?;
        result.push(PlaylistWithTracks { playlist, tracks });
    }
    Ok(result)
}

pub async fn detail(playlist_id: i32) -> Result<PlaylistWithTracks, String> {
    let playlist = db::get_playlist_by_id(playlist_id)
        .await
        .map_err(|err| err.to_string())?
        .ok_or_else(|| "Playlist not found".to_string())?;

    let tracks = db::list_track_playlist(playlist_id)
        .await
        .map_err(|err| err.to_string())?;

    Ok(PlaylistWithTracks { playlist, tracks })
}

pub async fn remove(playlist_id: i32) -> Result<(), String> {
    db::delete_playlist(playlist_id)
        .await
        .map_err(|err| err.to_string())
}

pub async fn remove_track(playlist_id: i32, track_id: &str) -> Result<(), String> {
    db::remove_from_playlist(playlist_id, track_id)
        .await
        .map_err(|err| err.to_string())
}
