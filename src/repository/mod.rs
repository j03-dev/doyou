pub mod db;
pub mod downloads;
pub mod favorites;
pub mod history;
pub mod identity;
pub mod local;
pub mod metadata;
pub mod playlist;
pub mod settings;
pub mod source;
pub mod youtube;
use anyhow::Result;

pub use source::TrackSource;

use rusql_alchemy::prelude::*;

use crate::core::platform;
use db::models::Track;
use yt::data_api::types::{Item, Snippet, Thumb, Thumbnails, VideoId};

/// Shared upsert: insert a track, refreshing mutable metadata if it exists.
pub(crate) async fn upsert_track(track: &Track) -> Result<Track> {
    let conn = db::conn().await.map_err(anyhow::Error::from_boxed)?;
    if let Some(mut existing) = Track::get(kwargs!(id = track.id.clone()), conn)
        .await
        .map_err(anyhow::Error::from_boxed)?
    {
        existing.title = track.title.clone();
        existing.channel_name = track.channel_name.clone();
        existing.thumbnail_url = track.thumbnail_url.clone();
        // Partial updates (for example from history or favorites) must not erase
        // the file path or extracted metadata of an existing local track.
        existing.album = track.album.clone().or(existing.album);
        existing.duration = track.duration.or(existing.duration);
        existing.cover_path = track.cover_path.clone().or(existing.cover_path);
        existing.src = track.src.clone().or(existing.src);
        existing
            .update(conn)
            .await
            .map_err(anyhow::Error::from_boxed)?;
        return Ok(existing);
    }
    track.save(conn).await.map_err(anyhow::Error::from_boxed)
}

pub async fn get_track(track_id: &str) -> Result<Option<Track>> {
    let conn = db::conn().await.map_err(anyhow::Error::from_boxed)?;
    Track::get(kwargs!(id = track_id), conn)
        .await
        .map_err(anyhow::Error::from_boxed)
}

/// Canonical track for an `Item` coming from the UI.
///
/// Local and downloaded rows are authoritative: an `Item` is only a partial
/// representation and must never overwrite their file path or extracted tags.
pub async fn ensure_track(item: &Item) -> Result<Track> {
    let track = item_to_track(item);
    if TrackSource::from_id(&track.id).is_local() {
        return get_track(&track.id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Local track not found"));
    }

    // A YouTube item with an available download is the same song: record the
    // offline variant everywhere so favorites, playlists and history don't
    // accumulate duplicates.
    if let Some(downloaded) = downloads::find_downloaded(&track.id).await? {
        return Ok(downloaded);
    }

    upsert_track(&track).await
}

pub fn item_to_track(item: &Item) -> Track {
    Track {
        id: item.id.as_string().unwrap_or_default(),
        title: item.snippet.title.clone(),
        channel_name: item.snippet.channel_title.clone(),
        thumbnail_url: item.snippet.thumbnails.high.url.clone(),
        ..Default::default()
    }
}

/// Prefers artwork extracted from the file, falling back to the remote URL.
pub fn track_thumbnail(track: &Track) -> String {
    track
        .cover_path
        .as_deref()
        .and_then(|path| platform::local_image_url(path).ok())
        .unwrap_or_else(|| track.thumbnail_url.clone())
}

pub fn track_to_item(track: &Track) -> Item {
    Item {
        id: VideoId::Literal(track.id.clone()),
        snippet: Snippet {
            title: track.title.clone(),
            channel_title: track.channel_name.clone(),
            thumbnails: Thumbnails {
                high: Thumb {
                    url: track_thumbnail(track),
                },
            },
        },
    }
}

pub fn tracks_to_items(tracks: &[Track]) -> Vec<Item> {
    tracks.iter().map(track_to_item).collect()
}
