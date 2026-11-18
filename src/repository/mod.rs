pub mod db;
pub mod downloads;
pub mod favorites;
pub mod history;
pub mod local;
pub mod metadata;
pub mod playlist;
pub mod settings;
pub mod source;
pub mod youtube;

pub use source::TrackSource;

use rusql_alchemy::prelude::*;

use crate::core::platform;
use db::models::Track;
use yt::data_api::types::{Item, Snippet, Thumb, Thumbnails, VideoId};

/// Shared upsert: insert a track, refreshing mutable metadata if it exists.
pub(crate) async fn upsert_track(track: &Track) -> Result<Track, String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    if let Some(mut existing) = Track::get(kwargs!(id = track.id.clone()), conn)
        .await
        .map_err(|err| err.to_string())?
    {
        existing.title = track.title.clone();
        existing.channel_name = track.channel_name.clone();
        existing.thumbnail_url = track.thumbnail_url.clone();
        existing.album = track.album.clone();
        existing.duration = track.duration;
        existing.cover_path = track.cover_path.clone();
        existing.src = track.src.clone();
        existing.update(conn).await.map_err(|err| err.to_string())?;
        return Ok(existing);
    }
    track.save(conn).await.map_err(|err| err.to_string())
}

pub async fn get_track(track_id: &str) -> Result<Option<Track>, String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    Track::get(kwargs!(id = track_id), conn)
        .await
        .map_err(|err| err.to_string())
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
        .and_then(|path| platform::local_media_url(path).ok())
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
