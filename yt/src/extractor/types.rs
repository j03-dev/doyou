use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub url: Option<String>,

    pub signature_cipher: Option<String>,

    pub mime_type: Option<String>,

    pub bitrate: Option<i64>,

    pub audio_quality: Option<String>,

    pub content_length: Option<String>,

    pub average_bitrate: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    pub formats: Option<Vec<Format>>,

    pub adaptive_formats: Option<Vec<Format>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayabilityStatus {
    pub status: String,

    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResponse {
    pub streaming_data: Option<StreamingData>,

    pub playability_status: PlayabilityStatus,
}
