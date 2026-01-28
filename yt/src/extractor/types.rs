use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Format {
    #[serde(rename = "url")]
    pub url: Option<String>,

    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,

    #[serde(rename = "bitrate")]
    pub bitrate: Option<i64>,

    #[serde(rename = "audioQuality")]
    pub audio_quality: Option<String>,

    #[serde(rename = "contentLength")]
    pub content_length: Option<String>,

    #[serde(rename = "averageBitrate")]
    pub average_bitrate: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct StreamingData {
    #[serde(rename = "formats")]
    pub formats: Option<Vec<Format>>,

    #[serde(rename = "adaptiveFormats")]
    pub adaptive_formats: Option<Vec<Format>>,
}

#[derive(Debug, Deserialize)]
pub struct PlayabilityStatus {
    pub status: String,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerResponse {
    #[serde(rename = "streamingData")]
    pub streaming_data: Option<StreamingData>,

    #[serde(rename = "playabilityStatus")]
    pub playability_status: PlayabilityStatus,
}
