use anyhow::Result;
use rusql_alchemy::prelude::*;
use yt::data_api::types::Item;

use super::db;
use super::db::models::{Playlist, Track, TrackPlaylist};
use super::{ensure_track, identity, track_thumbnail};

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

pub async fn create(name: &str) -> Result<Playlist> {
    let conn = db::conn().await.map_err(anyhow::Error::from_boxed)?;
    Playlist::create(kwargs!(name = name), conn)
        .await
        .map_err(anyhow::Error::from_boxed)
}

pub async fn add(item: &Item, playlist_id: i32) -> Result<Track> {
    let conn = db::conn().await.map_err(anyhow::Error::from_boxed)?;
    let track = ensure_track(item).await?;

    TrackPlaylist::create(
        kwargs!(track_id = track.id.clone(), playlist_id = playlist_id),
        conn,
    )
    .await
    .map_err(anyhow::Error::from_boxed)?;

    Ok(track)
}

pub async fn list() -> Result<Vec<PlaylistWithTracks>> {
    let conn = db::conn().await.map_err(anyhow::Error::from_boxed)?;
    let playlists: Vec<Playlist> = Playlist::all(conn)
        .await
        .map_err(anyhow::Error::from_boxed)?;

    let mut result = Vec::with_capacity(playlists.len());
    for playlist in playlists {
        let tracks = tracks_for(playlist.id, conn).await?;
        result.push(PlaylistWithTracks { playlist, tracks });
    }
    Ok(result)
}

pub async fn detail(playlist_id: i32) -> Result<PlaylistWithTracks> {
    let conn = db::conn().await.map_err(anyhow::Error::from_boxed)?;
    let playlist = Playlist::get(kwargs!(id = playlist_id), conn)
        .await
        .map_err(anyhow::Error::from_boxed)?
        .ok_or_else(|| anyhow::anyhow!("Playlist not found"))?;
    let tracks = tracks_for(playlist_id, conn).await?;

    Ok(PlaylistWithTracks { playlist, tracks })
}

pub async fn remove(playlist_id: i32) -> Result<()> {
    let conn = db::conn().await.map_err(anyhow::Error::from_boxed)?;
    let links: Vec<TrackPlaylist> =
        TrackPlaylist::filter(kwargs!(playlist_id == playlist_id), conn)
            .await
            .map_err(anyhow::Error::from_boxed)?;
    for link in links {
        link.delete(conn).await.map_err(anyhow::Error::from_boxed)?;
    }
    if let Some(playlist) = Playlist::get(kwargs!(id = playlist_id), conn)
        .await
        .map_err(anyhow::Error::from_boxed)?
    {
        playlist
            .delete(conn)
            .await
            .map_err(anyhow::Error::from_boxed)?;
    }
    Ok(())
}

pub async fn remove_track(playlist_id: i32, item: &Item) -> Result<()> {
    let conn = db::conn().await.map_err(anyhow::Error::from_boxed)?;
    let links: Vec<TrackPlaylist> =
        TrackPlaylist::filter(kwargs!(playlist_id == playlist_id), conn)
            .await
            .map_err(anyhow::Error::from_boxed)?;

    for link in links {
        let track = Track::get(kwargs!(id = link.track_id.clone()), conn)
            .await
            .map_err(anyhow::Error::from_boxed)?;
        if track.is_some_and(|track| identity::same_media(&track, item)) {
            link.delete(conn).await.map_err(anyhow::Error::from_boxed)?;
        }
    }
    Ok(())
}

async fn tracks_for(playlist_id: i32, conn: &Connection) -> Result<Vec<Track>> {
    let tracks: Vec<Track> = select!(Track, TrackPlaylist)
        .inner_join::<Track, TrackPlaylist>(kwargs!(Track.id == TrackPlaylist.track_id))
        .r#where(kwargs!(TrackPlaylist.playlist_id == playlist_id))
        .fetch_all(conn)
        .await
        .map_err(anyhow::Error::from_boxed)?;
    Ok(identity::dedupe(tracks))
}
