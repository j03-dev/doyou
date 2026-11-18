use anyhow::Result;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::Path;

use rusql_alchemy::prelude::*;
use walkdir::WalkDir;

use crate::core::platform;
use crate::repository::TrackSource;
use crate::repository::db;
use crate::repository::db::models::Track;
use crate::repository::metadata;
use crate::repository::upsert_track;
use doyou_media as media;

fn track_id_for(path: &Path) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    path.to_string_lossy().hash(&mut hasher);
    TrackSource::Local.id(&format!("{:x}", hasher.finish()))
}

fn track_from_path(path: &Path) -> Track {
    let metadata = metadata::read(path);
    let id = track_id_for(path);

    Track {
        title: metadata.title.unwrap_or_else(|| fallback_title(path)),
        channel_name: metadata.artist.unwrap_or_else(|| fallback_artist(path)),
        album: metadata.album,
        duration: metadata.duration,
        cover_path: metadata
            .picture
            .as_ref()
            .and_then(|cover| metadata::save_cover(&id, cover).ok()),
        id,
        thumbnail_url: String::new(),
        downloaded_at: None,
        src: Some(path.to_string_lossy().to_string()),
    }
}

fn fallback_title(path: &Path) -> String {
    path.file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown")
        .to_string()
}

fn fallback_artist(path: &Path) -> String {
    path.parent()
        .and_then(|parent| parent.file_name())
        .and_then(|name| name.to_str())
        .unwrap_or("Local")
        .to_string()
}

fn local_tracks(tracks: Vec<Track>) -> Vec<Track> {
    tracks
        .into_iter()
        .filter(|track| TrackSource::from_id(&track.id).is_local())
        .collect()
}

async fn all_db_tracks() -> Result<Vec<Track>> {
    let conn = db::conn().await.map_err(anyhow::Error::from_boxed)?;
    Track::all(conn).await.map_err(anyhow::Error::from_boxed)
}

/// Scans the local music folders and syncs the database:
/// inserts tracks that are new on disk, deletes rows whose file is gone.
pub async fn scan() -> Result<Vec<Track>> {
    let mut found: HashMap<String, Track> = HashMap::new();
    for dir in platform::local_music_dirs() {
        let entries = WalkDir::new(dir).follow_links(false).max_depth(6);
        for entry in entries.into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if !media::is_audio(path) {
                continue;
            }
            let track = track_from_path(path);
            found.insert(track.id.clone(), track);
        }
    }

    let existing = local_tracks(all_db_tracks().await?);

    for track in found.values() {
        upsert_track(track).await?;
    }

    let conn = db::conn().await.map_err(anyhow::Error::from_boxed)?;
    for track in &existing {
        // Downloads live in the app-private folder and are not scanned.
        if track.downloaded_at.is_some() {
            continue;
        }
        if !found.contains_key(&track.id)
            && let Some(stored) = Track::get(kwargs!(id = track.id.clone()), conn)
                .await
                .map_err(anyhow::Error::from_boxed)?
        {
            stored
                .delete(conn)
                .await
                .map_err(anyhow::Error::from_boxed)?;
        }
    }

    all().await
}

pub async fn all() -> Result<Vec<Track>> {
    Ok(local_tracks(all_db_tracks().await?))
}

pub async fn search(query: &str) -> Result<Vec<Track>> {
    let query = query.to_lowercase();
    let tracks = all_db_tracks()
        .await?
        .into_iter()
        .filter(|track| TrackSource::from_id(&track.id).is_local())
        .filter(|t| {
            t.title.to_lowercase().contains(&query)
                || t.channel_name.to_lowercase().contains(&query)
                || t.src
                    .as_deref()
                    .and_then(|s| Path::new(s).file_name())
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.to_lowercase().contains(&query))
        })
        .collect::<Vec<_>>();
    Ok(tracks)
}

pub fn src_url(path: &str) -> Result<String> {
    platform::local_media_url(path).map_err(anyhow::Error::from_boxed)
}
