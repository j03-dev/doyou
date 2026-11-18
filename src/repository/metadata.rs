use std::path::Path;

use lofty::file::{AudioFile, TaggedFileExt};
use lofty::picture::PictureType;
use lofty::prelude::ItemKey;

use crate::core::platform;
use doyou_media::ImageFormat;

#[derive(Debug, Clone, Default)]
pub struct TrackMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<f64>,
    pub picture: Option<CoverArt>,
}

#[derive(Debug, Clone)]
pub struct CoverArt {
    pub data: Vec<u8>,
    pub format: ImageFormat,
}

/// Reads tags, duration and embedded artwork from a local audio file.
/// Unreadable or unsupported files simply yield no metadata.
pub fn read(path: &Path) -> TrackMetadata {
    let Ok(tagged) = lofty::read_from_path(path) else {
        return TrackMetadata::default();
    };

    let tag = tagged
        .tag(tagged.primary_tag_type())
        .or_else(|| tagged.tags().first());

    let duration = {
        let duration = tagged.properties().duration();
        (!duration.is_zero()).then_some(duration.as_secs_f64())
    };

    let Some(tag) = tag else {
        return TrackMetadata {
            duration,
            ..Default::default()
        };
    };

    let picture = tag
        .get_picture_type(PictureType::CoverFront)
        .or_else(|| tag.pictures().first())
        .map(|picture| CoverArt {
            data: picture.data().to_vec(),
            format: picture
                .mime_type()
                .and_then(|mime| ImageFormat::from_mime(mime.as_str()))
                .unwrap_or(ImageFormat::Jpeg),
        });

    TrackMetadata {
        title: non_empty(tag.get_string(ItemKey::TrackTitle)),
        artist: non_empty(tag.get_string(ItemKey::TrackArtist)),
        album: non_empty(tag.get_string(ItemKey::AlbumTitle)),
        duration,
        picture,
    }
}

/// Persists embedded artwork next to the app database and returns its file path.
pub fn save_cover(track_id: &str, cover: &CoverArt) -> Result<String, String> {
    let directory = platform::covers_dir().map_err(|err| err.to_string())?;
    let path = directory.join(format!(
        "{}.{}",
        file_name_part(track_id),
        cover.format.as_str()
    ));
    std::fs::write(&path, &cover.data).map_err(|err| err.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

fn non_empty(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

pub fn file_name_part(value: &str) -> String {
    let cleaned: String = value
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    let cleaned = cleaned.trim_matches('_');
    if cleaned.is_empty() {
        "cover".to_string()
    } else {
        cleaned.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitizes_unsafe_file_names() {
        assert_eq!(file_name_part("local:yt-abc/123"), "local_yt_abc_123");
        assert_eq!(file_name_part("///"), "cover");
    }

    #[test]
    fn drops_blank_tag_values() {
        assert_eq!(non_empty(Some("  ")), None);
        assert_eq!(non_empty(Some(" Title ")).as_deref(), Some("Title"));
    }
}
