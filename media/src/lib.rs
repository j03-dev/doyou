//! Shared media types. This is the lowest layer: no app, UI or network deps.

use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioFormat {
    Mp3,
    M4a,
    Flac,
    Ogg,
    Opus,
    Wav,
    Aac,
}

impl AudioFormat {
    pub const ALL: [Self; 7] = [
        Self::Mp3,
        Self::M4a,
        Self::Flac,
        Self::Ogg,
        Self::Opus,
        Self::Wav,
        Self::Aac,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mp3 => "mp3",
            Self::M4a => "m4a",
            Self::Flac => "flac",
            Self::Ogg => "ogg",
            Self::Opus => "opus",
            Self::Wav => "wav",
            Self::Aac => "aac",
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Mp3 => "audio/mpeg",
            Self::M4a => "audio/mp4",
            Self::Flac => "audio/flac",
            Self::Ogg => "audio/ogg",
            Self::Opus => "audio/ogg; codecs=opus",
            Self::Wav => "audio/wav",
            Self::Aac => "audio/aac",
        }
    }

    pub fn from_extension(extension: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|format| format.as_str() == extension)
    }

    /// Accepts both plain MIME values and YouTube style `audio/webm; codecs=opus`.
    pub fn from_mime(mime: &str) -> Option<Self> {
        let value = mime.trim();
        if let Some(format) = Self::ALL
            .into_iter()
            .find(|format| format.mime_type() == value)
        {
            return Some(format);
        }
        let base = value.split(';').next().unwrap_or(value).trim();
        if let Some(format) = Self::ALL
            .into_iter()
            .find(|format| format.mime_type() == base)
        {
            return Some(format);
        }
        if base.contains("webm") || mime.contains("opus") {
            Some(Self::Opus)
        } else if base.contains("mpeg") {
            Some(Self::Mp3)
        } else if base.contains("mp4") {
            Some(Self::M4a)
        } else {
            None
        }
    }

    pub fn from_path(path: &Path) -> Option<Self> {
        extension(path).and_then(|extension| Self::from_extension(&extension))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    Jpeg,
    Png,
    Webp,
    Gif,
    Bmp,
    Tiff,
}

impl ImageFormat {
    pub const ALL: [Self; 6] = [
        Self::Jpeg,
        Self::Png,
        Self::Webp,
        Self::Gif,
        Self::Bmp,
        Self::Tiff,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Webp => "webp",
            Self::Gif => "gif",
            Self::Bmp => "bmp",
            Self::Tiff => "tif",
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Webp => "image/webp",
            Self::Gif => "image/gif",
            Self::Bmp => "image/bmp",
            Self::Tiff => "image/tiff",
        }
    }

    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "png" => Some(Self::Png),
            "webp" => Some(Self::Webp),
            "gif" => Some(Self::Gif),
            "bmp" => Some(Self::Bmp),
            "tif" | "tiff" => Some(Self::Tiff),
            _ => None,
        }
    }

    pub fn from_mime(mime: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|format| format.mime_type() == mime)
    }

    pub fn from_path(path: &Path) -> Option<Self> {
        extension(path).and_then(|extension| Self::from_extension(&extension))
    }
}

/// A file format the app knows how to serve to the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MediaFormat {
    Audio(AudioFormat),
    Image(ImageFormat),
}

impl MediaFormat {
    pub fn from_path(path: &Path) -> Option<Self> {
        AudioFormat::from_path(path)
            .map(Self::Audio)
            .or_else(|| ImageFormat::from_path(path).map(Self::Image))
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Audio(format) => format.mime_type(),
            Self::Image(format) => format.mime_type(),
        }
    }
}

pub fn is_audio(path: &Path) -> bool {
    AudioFormat::from_path(path).is_some()
}

fn extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_formats_round_trip() {
        for format in AudioFormat::ALL {
            assert_eq!(AudioFormat::from_extension(format.as_str()), Some(format));
            assert_eq!(AudioFormat::from_mime(format.mime_type()), Some(format));
            assert_eq!(
                AudioFormat::from_path(&Path::new("song").with_extension(format.as_str())),
                Some(format)
            );
        }
    }

    #[test]
    fn image_formats_round_trip() {
        for format in ImageFormat::ALL {
            assert_eq!(ImageFormat::from_extension(format.as_str()), Some(format));
            assert_eq!(ImageFormat::from_mime(format.mime_type()), Some(format));
        }
        assert_eq!(ImageFormat::from_extension("jpeg"), Some(ImageFormat::Jpeg));
    }

    #[test]
    fn youtube_mime_types_resolve() {
        assert_eq!(
            AudioFormat::from_mime("audio/webm; codecs=opus"),
            Some(AudioFormat::Opus)
        );
        assert_eq!(
            AudioFormat::from_mime("audio/mp4; codecs=mp4a.40.2"),
            Some(AudioFormat::M4a)
        );
    }

    #[test]
    fn media_format_detects_both_kinds() {
        assert_eq!(
            MediaFormat::from_path(Path::new("song.MP3")),
            Some(MediaFormat::Audio(AudioFormat::Mp3))
        );
        assert_eq!(
            MediaFormat::from_path(Path::new("cover.jpeg")),
            Some(MediaFormat::Image(ImageFormat::Jpeg))
        );
        assert_eq!(MediaFormat::from_path(Path::new("notes.txt")), None);
    }
}
