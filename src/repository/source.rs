/// Where a track comes from. Encodes itself into the id stored in the database,
/// so playback and filtering never need to compare raw string prefixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrackSource {
    YouTube,
    Local,
    Downloaded,
}

impl TrackSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::YouTube => "youtube",
            Self::Local => "local",
            Self::Downloaded => "download",
        }
    }

    pub fn from_id(id: &str) -> Self {
        let prefix = format!("{}:", Self::Local.as_str());
        let Some(key) = id.strip_prefix(&prefix) else {
            return Self::YouTube;
        };

        let download_key = format!("{}-", Self::Downloaded.as_str());
        if key.starts_with(&download_key) {
            Self::Downloaded
        } else {
            Self::Local
        }
    }

    pub fn id(&self, key: &str) -> String {
        match self {
            Self::YouTube => key.to_string(),
            Self::Local => format!("{}:{}", Self::Local.as_str(), key),
            // Downloaded files are local files too, so playback resolves them
            // through the same local-file path as scanned music.
            Self::Downloaded => Self::Local.id(&format!("{}-{}", Self::Downloaded.as_str(), key)),
        }
    }

    pub fn is_local(&self) -> bool {
        matches!(self, Self::Local | Self::Downloaded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_str_matches_variant_names() {
        assert_eq!(TrackSource::YouTube.as_str(), "youtube");
        assert_eq!(TrackSource::Local.as_str(), "local");
        assert_eq!(TrackSource::Downloaded.as_str(), "download");
    }

    #[test]
    fn ids_round_trip() {
        for source in [
            TrackSource::YouTube,
            TrackSource::Local,
            TrackSource::Downloaded,
        ] {
            let id = source.id("abc123");
            assert_eq!(TrackSource::from_id(&id), source);
        }
    }

    #[test]
    fn downloaded_ids_are_local() {
        let id = TrackSource::Downloaded.id("abc123");
        assert!(TrackSource::from_id(&id).is_local());
        assert!(!TrackSource::from_id("abc123").is_local());
    }
}
