use anyhow::Result;
use std::path::{Path, PathBuf};

use doyou_media::AudioFormat;
use rusql_alchemy::prelude::*;
use yt::data_api::types::Item;
use yt::extractor::YouTubeExtractor;

use crate::core::platform;
use crate::repository::TrackSource;
use crate::repository::db;
use crate::repository::db::models::Track;
use crate::repository::metadata;
use crate::repository::upsert_track;

pub fn is_downloaded(track: &Track) -> bool {
    TrackSource::from_id(&track.id) == TrackSource::Downloaded
}

pub async fn find_downloaded(video_id: &str) -> Result<Option<Track>> {
    get(&TrackSource::Downloaded.id(video_id)).await
}

async fn get(track_id: &str) -> Result<Option<Track>> {
    let conn = db::conn().await?;
    Ok(Track::get(kwargs!(id = track_id), conn).await?)
}

pub async fn list() -> Result<Vec<Track>> {
    let conn = db::conn().await?;
    let tracks: Vec<Track> = Track::all(conn).await?;
    let mut result = Vec::new();

    for track in tracks.into_iter().filter(is_downloaded) {
        if file_exists(&track) {
            result.push(track);
            continue;
        }

        let Some(restored) = restore_src(conn, track.clone()).await? else {
            // No file on disk: drop the orphan so it can be downloaded again.
            track.delete(conn).await?;
            continue;
        };
        result.push(restored);
    }

    Ok(result)
}

pub async fn download(item: &Item) -> Result<Track> {
    let video_id = item
        .id
        .as_string()
        .ok_or_else(|| anyhow::anyhow!("This track does not have a YouTube id"))?;
    let track_id = TrackSource::Downloaded.id(&video_id);
    let conn = db::conn().await?;

    if let Some(track) = Track::get(kwargs!(id = track_id.clone()), conn).await? {
        if file_exists(&track) {
            return Ok(track);
        }
        if let Some(restored) = restore_src(conn, track).await? {
            return Ok(restored);
        }
    }

    let title = item.snippet.title.clone();
    let artist = item.snippet.channel_title.clone();
    let directory = platform::downloads_dir()?;
    let stem = format!(
        "{} - {}",
        metadata::file_name_part(&title),
        metadata::file_name_part(&artist)
    );

    let extractor = YouTubeExtractor::new()?;
    let path = extractor
        .download_audio(&video_id, &directory, &stem)
        .await?;

    // The rest of the app (scanner, media server, player) only understands
    // formats from our enum, so reject anything else before storing it.
    AudioFormat::from_path(&path).ok_or_else(|| {
        anyhow::anyhow!("unsupported downloaded audio format: {}", path.display())
    })?;

    let metadata = metadata::read(&path);
    let cover_path = metadata
        .picture
        .as_ref()
        .and_then(|cover| metadata::save_cover(&track_id, cover).ok());

    let track = Track {
        id: track_id,
        title: metadata.title.unwrap_or(title),
        channel_name: metadata.artist.unwrap_or(artist),
        thumbnail_url: item.snippet.thumbnails.high.url.clone(),
        album: metadata.album,
        duration: metadata.duration,
        cover_path,
        downloaded_at: Some(now()),
        src: Some(path.to_string_lossy().to_string()),
    };

    upsert_track(&track).await.inspect_err(|_| {
        let _ = std::fs::remove_file(path);
    })
}

pub async fn remove(track_id: &str) -> Result<()> {
    let conn = db::conn().await?;
    let Some(track) = Track::get(kwargs!(id = track_id), conn).await? else {
        return Ok(());
    };

    // Never delete scanned local files through the downloads API.
    if !is_downloaded(&track) {
        return Ok(());
    }

    if let Some(path) = track.src.as_deref() {
        let _ = std::fs::remove_file(path);
    }
    if let Some(path) = track.cover_path.as_deref() {
        let _ = std::fs::remove_file(path);
    }
    Ok(track.delete(conn).await?)
}

fn file_exists(track: &Track) -> bool {
    track
        .src
        .as_deref()
        .is_some_and(|path| Path::new(path).is_file())
}

/// Older builds could clear `src` when a downloaded track was played or added
/// to favorites. Recover the path from the original download filename.
async fn restore_src(conn: &Connection, mut track: Track) -> Result<Option<Track>> {
    let directory = platform::downloads_dir()?;
    let stem = format!(
        "{} - {}",
        metadata::file_name_part(&track.title),
        metadata::file_name_part(&track.channel_name)
    );

    let found = std::fs::read_dir(directory)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path: &PathBuf| {
            AudioFormat::from_path(path).is_some()
                && path.file_stem().and_then(|stem| stem.to_str()) == Some(stem.as_str())
        });

    let Some(path) = found else {
        return Ok(None);
    };

    track.src = Some(path.to_string_lossy().to_string());
    track.update(conn).await?;
    Ok(Some(track))
}

fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs_f64())
        .unwrap_or(0.0)
}
