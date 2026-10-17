use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::Path;

use walkdir::WalkDir;

use crate::core::platform;
use crate::repository::db;
use crate::repository::db::models::Track;

pub const LOCAL_PREFIX: &str = "local:";

const AUDIO_EXTENSIONS: [&str; 7] = ["mp3", "m4a", "flac", "ogg", "opus", "wav", "aac"];

pub fn is_local_id(id: &str) -> bool {
    id.starts_with(LOCAL_PREFIX)
}

fn track_id_for(path: &Path) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    path.to_string_lossy().hash(&mut hasher);
    format!("{LOCAL_PREFIX}{:x}", hasher.finish())
}

fn is_audio(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .is_some_and(|ext| AUDIO_EXTENSIONS.contains(&ext.as_str()))
}

fn track_from_path(path: &Path) -> Track {
    let title = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown")
        .to_string();
    let channel_name = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("Local")
        .to_string();

    Track {
        track_id: track_id_for(path),
        title,
        channel_name,
        thumbnail_url: String::new(),
        src: Some(path.to_string_lossy().to_string()),
    }
}

fn local_tracks(tracks: Vec<Track>) -> Vec<Track> {
    tracks
        .into_iter()
        .filter(|t| is_local_id(&t.track_id))
        .collect()
}

/// Scans the local music folders and syncs the database:
/// inserts tracks that are new on disk, deletes rows whose file is gone.
pub async fn scan() -> Result<Vec<Track>, String> {
    let mut found: HashMap<String, Track> = HashMap::new();
    for dir in platform::local_music_dirs() {
        let entries = WalkDir::new(dir).follow_links(false).max_depth(6);
        for entry in entries.into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if !is_audio(path) {
                continue;
            }
            let track = track_from_path(path);
            found.insert(track.track_id.clone(), track);
        }
    }

    let existing = local_tracks(db::get_all_tracks().await.map_err(|e| e.to_string())?);

    for track in found.values() {
        db::save_track(track).await.map_err(|e| e.to_string())?;
    }

    for track in &existing {
        if !found.contains_key(&track.track_id) {
            db::delete_track(&track.track_id)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    all().await
}

pub async fn all() -> Result<Vec<Track>, String> {
    let tracks = db::get_all_tracks().await.map_err(|e| e.to_string())?;
    Ok(local_tracks(tracks))
}

pub async fn search(query: &str) -> Result<Vec<Track>, String> {
    let query = query.to_lowercase();
    let tracks = db::get_all_tracks()
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .filter(|t| is_local_id(&t.track_id))
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

pub fn src_url(path: &str) -> Result<String, String> {
    platform::local_media_url(path).map_err(|e| e.to_string())
}
