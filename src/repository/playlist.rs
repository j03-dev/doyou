use rusql_alchemy::prelude::*;
use yt::data_api::types::Item;

use super::db;
use super::db::models::{Playlist, Track, TrackPlaylist};
use super::{ensure_track, track_thumbnail};

#[derive(Debug, Clone, PartialEq)]
pub struct PlaylistWithTracks {
    pub playlist: Playlist,
    pub tracks: Vec<Track>,
}

impl PlaylistWithTracks {
    pub fn thumbnails(&self) -> Vec<String> {
        self.tracks.iter().take(4).map(track_thumbnail).collect()
    }
}

pub async fn create(name: &str) -> Result<Playlist, String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    Playlist::create(kwargs!(name = name), conn)
        .await
        .map_err(|err| err.to_string())
}

pub async fn add(item: &Item, playlist_id: i32) -> Result<Track, String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let track = ensure_track(item).await?;

    TrackPlaylist::create(
        kwargs!(track_id = track.id.clone(), playlist_id = playlist_id),
        conn,
    )
    .await
    .map_err(|err| err.to_string())?;

    Ok(track)
}

pub async fn list() -> Result<Vec<PlaylistWithTracks>, String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let playlists: Vec<Playlist> = Playlist::all(conn).await.map_err(|err| err.to_string())?;

    let mut result = Vec::with_capacity(playlists.len());
    for playlist in playlists {
        let tracks = tracks_for(playlist.id, conn).await?;
        result.push(PlaylistWithTracks { playlist, tracks });
    }
    Ok(result)
}

pub async fn detail(playlist_id: i32) -> Result<PlaylistWithTracks, String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let playlist = Playlist::get(kwargs!(id = playlist_id), conn)
        .await
        .map_err(|err| err.to_string())?
        .ok_or_else(|| "Playlist not found".to_string())?;
    let tracks = tracks_for(playlist_id, conn).await?;

    Ok(PlaylistWithTracks { playlist, tracks })
}

pub async fn remove(playlist_id: i32) -> Result<(), String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let links: Vec<TrackPlaylist> =
        TrackPlaylist::filter(kwargs!(playlist_id == playlist_id), conn)
            .await
            .map_err(|err| err.to_string())?;
    for link in links {
        link.delete(conn).await.map_err(|err| err.to_string())?;
    }
    if let Some(playlist) = Playlist::get(kwargs!(id = playlist_id), conn)
        .await
        .map_err(|err| err.to_string())?
    {
        playlist.delete(conn).await.map_err(|err| err.to_string())?;
    }
    Ok(())
}

pub async fn remove_track(playlist_id: i32, track_id: &str) -> Result<(), String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let links: Vec<TrackPlaylist> = TrackPlaylist::filter(
        kwargs!(playlist_id == playlist_id).and(kwargs!(track_id == track_id)),
        conn,
    )
    .await
    .map_err(|err| err.to_string())?;
    for link in links {
        link.delete(conn).await.map_err(|err| err.to_string())?;
    }
    Ok(())
}

async fn tracks_for(playlist_id: i32, conn: &Connection) -> Result<Vec<Track>, String> {
    let tracks: Vec<Track> = select!(Track, TrackPlaylist)
        .inner_join::<Track, TrackPlaylist>(kwargs!(Track.id == TrackPlaylist.track_id))
        .r#where(kwargs!(TrackPlaylist.playlist_id == playlist_id))
        .fetch_all(conn)
        .await
        .map_err(|err| err.to_string())?;
    Ok(tracks)
}
