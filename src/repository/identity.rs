use std::borrow::Cow;
use std::path::Path;

use yt::data_api::types::Item;

use super::TrackSource;
use super::db::models::Track;

/// Anything that represents a playable song, regardless of where it came from.
pub trait MediaIdentity {
    fn media_id(&self) -> Cow<'_, str>;
    fn media_title(&self) -> &str;
    fn media_artist(&self) -> &str;
    fn media_src(&self) -> Option<&str>;
}

impl MediaIdentity for Track {
    fn media_id(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn media_title(&self) -> &str {
        &self.title
    }

    fn media_artist(&self) -> &str {
        &self.channel_name
    }

    fn media_src(&self) -> Option<&str> {
        self.src.as_deref()
    }
}

impl MediaIdentity for Item {
    fn media_id(&self) -> Cow<'_, str> {
        Cow::Owned(self.id.as_string().unwrap_or_default())
    }

    fn media_title(&self) -> &str {
        &self.snippet.title
    }

    fn media_artist(&self) -> &str {
        &self.snippet.channel_title
    }

    fn media_src(&self) -> Option<&str> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MediaKey {
    YouTube(String),
    Meta(String),
}

/// Stable identity used to merge the same song from different sources.
pub fn key(value: &impl MediaIdentity) -> MediaKey {
    let id = value.media_id();
    if let Some(video_id) = TrackSource::video_id(&id) {
        return MediaKey::YouTube(video_id.to_string());
    }
    if TrackSource::from_id(&id) == TrackSource::YouTube {
        return MediaKey::YouTube(id.into_owned());
    }
    MediaKey::Meta(fingerprint(value).unwrap_or_else(|| format!("id:{}", value.media_id())))
}

/// Two entries are the same song when they share a YouTube id, or when a local
/// entry matches a remote one by title and artist. Two plain YouTube entries
/// are never merged just because their titles match.
pub fn same_media(a: &impl MediaIdentity, b: &impl MediaIdentity) -> bool {
    if key(a) == key(b) {
        return true;
    }

    let a_id = a.media_id();
    let b_id = b.media_id();
    if TrackSource::from_id(&a_id) == TrackSource::YouTube
        && TrackSource::from_id(&b_id) == TrackSource::YouTube
    {
        return false;
    }

    match (fingerprint(a), fingerprint(b)) {
        (Some(a), Some(b)) => a == b,
        _ => false,
    }
}

/// Downloads beat scanned local files, which beat streamed YouTube entries.
pub fn rank(value: &impl MediaIdentity) -> u8 {
    let id = value.media_id();
    match TrackSource::from_id(&id) {
        TrackSource::Downloaded => 3,
        TrackSource::Local => 2,
        TrackSource::YouTube => 1,
    }
}

pub fn prefer<T: MediaIdentity + Clone>(a: T, b: T) -> T {
    if rank(&b) > rank(&a) { b } else { a }
}

/// Removes duplicate songs, keeping the most useful variant of each one.
pub fn dedupe<T: MediaIdentity + Clone>(values: Vec<T>) -> Vec<T> {
    let mut result: Vec<T> = Vec::with_capacity(values.len());
    for value in values {
        if let Some(index) = result
            .iter()
            .position(|existing| same_media(existing, &value))
        {
            let existing = result[index].clone();
            result[index] = prefer(existing, value);
        } else {
            result.push(value);
        }
    }
    result
}

fn fingerprint(value: &impl MediaIdentity) -> Option<String> {
    let title = if normalize(value.media_title()).is_empty() {
        value
            .media_src()
            .and_then(|src| Path::new(src).file_stem())
            .and_then(|stem| stem.to_str())
            .unwrap_or_default()
    } else {
        value.media_title()
    };
    let title = normalize(title);
    let artist = normalize(value.media_artist());
    if title.is_empty() || artist.is_empty() {
        return None;
    }
    Some(format!("{title}|{artist}"))
}

fn normalize(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn track(id: &str, title: &str, artist: &str) -> Track {
        Track {
            id: id.to_string(),
            title: title.to_string(),
            channel_name: artist.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn youtube_and_downloaded_variants_share_a_key() {
        let youtube = track("abc123", "Song", "Artist");
        let downloaded = track(&TrackSource::Downloaded.id("abc123"), "Song", "Artist");
        assert_eq!(key(&youtube), key(&downloaded));
        assert!(same_media(&youtube, &downloaded));
    }

    #[test]
    fn distinct_youtube_videos_do_not_merge_by_title() {
        let first = track("aaaaaaaaaaa", "Song", "Artist");
        let second = track("bbbbbbbbbbb", "Song", "Artist");
        assert!(!same_media(&first, &second));
    }

    #[test]
    fn local_files_match_remote_songs_by_metadata() {
        let local = track(&TrackSource::Local.id("hash"), "Song", "Artist");
        let youtube = track("abc123", "Song", "Artist");
        assert!(same_media(&local, &youtube));
    }

    #[test]
    fn dedupe_prefers_downloaded_variants() {
        let youtube = track("abc123", "Song", "Artist");
        let downloaded = track(&TrackSource::Downloaded.id("abc123"), "Song", "Artist");
        let result = dedupe(vec![youtube, downloaded]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, TrackSource::Downloaded.id("abc123"));
    }

    #[test]
    fn file_name_fallback_matches_remote_songs() {
        let mut local = track(&TrackSource::Local.id("hash"), "", "Artist");
        local.src = Some("/music/My Song.mp3".to_string());
        let youtube = track("abc123", "My Song", "Artist");
        assert!(same_media(&local, &youtube));
    }

    #[test]
    fn missing_title_or_artist_never_matches() {
        let first = track(&TrackSource::Local.id("one"), "", "Artist");
        let second = track(&TrackSource::Local.id("two"), "", "Artist");
        assert!(!same_media(&first, &second));
    }
}
