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

pub async fn list() -> Result<Vec<Track>, String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let tracks: Vec<Track> = Track::all(conn).await.map_err(|err| err.to_string())?;
    Ok(tracks.into_iter().filter(is_downloaded).collect())
}

pub async fn download(item: &Item) -> Result<Track, String> {
    let video_id = item
        .id
        .as_string()
        .ok_or_else(|| "This track does not have a YouTube id".to_string())?;
    let track_id = TrackSource::Downloaded.id(&video_id);

    if let Some(track) = crate::repository::get_track(&track_id).await? {
        return Ok(track);
    }

    let title = item.snippet.title.clone();
    let artist = item.snippet.channel_title.clone();
    let directory = platform::downloads_dir().map_err(|err| err.to_string())?;
    let stem = format!(
        "{} - {}",
        metadata::file_name_part(&title),
        metadata::file_name_part(&artist)
    );

    let extractor = YouTubeExtractor::new().map_err(|err| err.to_string())?;
    let path = extractor
        .download_audio(&video_id, &directory, &stem)
        .await
        .map_err(|err| err.to_string())?;

    // The rest of the app (scanner, media server, player) only understands
    // formats from our enum, so reject anything else before storing it.
    AudioFormat::from_path(&path)
        .ok_or_else(|| format!("unsupported downloaded audio format: {}", path.display()))?;

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

pub async fn remove(track_id: &str) -> Result<(), String> {
    let conn = db::conn().await.map_err(|err| err.to_string())?;
    let Some(track) = Track::get(kwargs!(id = track_id), conn)
        .await
        .map_err(|err| err.to_string())?
    else {
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
    track.delete(conn).await.map_err(|err| err.to_string())
}

fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs_f64())
        .unwrap_or(0.0)
}
