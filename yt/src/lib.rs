use anyhow::{Context, Result};
use reqwest;
use serde::Deserialize;
use serde_json::json;

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct Format {
    #[serde(rename = "itag")]
    pub itag: i32,

    #[serde(rename = "url")]
    pub url: Option<String>,

    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,

    #[serde(rename = "bitrate")]
    pub bitrate: Option<i64>,

    #[serde(rename = "width")]
    pub width: Option<i32>,

    #[serde(rename = "height")]
    pub height: Option<i32>,

    #[serde(rename = "quality")]
    pub quality: Option<String>,

    #[serde(rename = "qualityLabel")]
    pub quality_label: Option<String>,

    #[serde(rename = "audioQuality")]
    pub audio_quality: Option<String>,

    #[serde(rename = "audioSampleRate")]
    pub audio_sample_rate: Option<String>,

    #[serde(rename = "contentLength")]
    pub content_length: Option<String>,

    #[serde(rename = "averageBitrate")]
    pub average_bitrate: Option<i64>,

    #[serde(rename = "audioChannels")]
    pub audio_channels: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct StreamingData {
    #[serde(rename = "formats")]
    formats: Option<Vec<Format>>,

    #[serde(rename = "adaptiveFormats")]
    adaptive_formats: Option<Vec<Format>>,
}

#[derive(Debug, Deserialize)]
struct PlayabilityStatus {
    status: String,
    reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PlayerResponse {
    #[serde(rename = "streamingData")]
    streaming_data: Option<StreamingData>,

    #[serde(rename = "playabilityStatus")]
    playability_status: PlayabilityStatus,
}

pub struct YouTubeExtractor {
    client: reqwest::Client,
}

impl YouTubeExtractor {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .build()
            .expect("Failed to build HTTP client");

        Self { client }
    }

    /// Extract video ID from various YouTube URL formats
    pub fn extract_video_id(url: &str) -> Result<String> {
        use regex::Regex;

        let patterns = vec![
            r"(?:youtube\.com/watch\?v=|youtu\.be/)([a-zA-Z0-9_-]{11})",
            r"youtube\.com/embed/([a-zA-Z0-9_-]{11})",
            r"youtube\.com/v/([a-zA-Z0-9_-]{11})",
        ];

        for pattern in patterns {
            let re = Regex::new(pattern)?;
            if let Some(caps) = re.captures(url) {
                return Ok(caps[1].to_string());
            }
        }

        if url.len() == 11
            && url
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Ok(url.to_string());
        }

        anyhow::bail!("Could not extract video ID from URL: {}", url);
    }

    pub async fn get_formats(&self, url: &str) -> Result<Vec<Format>> {
        let video_id = Self::extract_video_id(url)?;

        let player_response = self.fetch_player_response(&video_id).await?;

        if player_response.playability_status.status != "OK" {
            let reason = player_response
                .playability_status
                .reason
                .unwrap_or_else(|| "Unknown error".to_string());
            anyhow::bail!("Video not playable: {}", reason);
        }

        let streaming_data = player_response
            .streaming_data
            .ok_or_else(|| anyhow::anyhow!("No streaming data found"))?;

        let mut all_formats = Vec::new();

        if let Some(formats) = streaming_data.formats {
            all_formats.extend(formats);
        }

        if let Some(adaptive_formats) = streaming_data.adaptive_formats {
            all_formats.extend(adaptive_formats);
        }

        let available_formats: Vec<Format> = all_formats
            .into_iter()
            .filter(|f| f.url.is_some())
            .collect();

        if available_formats.is_empty() {
            anyhow::bail!("No formats with direct URLs found");
        }

        Ok(available_formats)
    }

    async fn fetch_player_response(&self, video_id: &str) -> Result<PlayerResponse> {
        let api_key = "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8"; // Public API key

        let url = format!("https://www.youtube.com/youtubei/v1/player?key={}", api_key);

        let body = json!({
            "context": {
                "client": {
                    "clientName": "ANDROID",
                    "clientVersion": "19.09.37",
                    "androidSdkVersion": 30,
                    "hl": "en",
                    "gl": "US",
                    "utcOffsetMinutes": 0
                }
            },
            "videoId": video_id,
            "contentCheckOk": true,
            "racyCheckOk": true
        });

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("X-YouTube-Client-Name", "3")
            .header("X-YouTube-Client-Version", "19.09.37")
            .json(&body)
            .send()
            .await
            .context("Failed to fetch player response from InnerTube API")?;

        if !response.status().is_success() {
            anyhow::bail!("API request failed with status: {}", response.status());
        }

        let player_response: PlayerResponse = response
            .json()
            .await
            .context("Failed to parse player response JSON")?;

        Ok(player_response)
    }

    pub async fn get_best_audio_url(&self, url: &str) -> Result<String> {
        let formats = self.get_formats(url).await?;

        let audio_formats: Vec<&Format> =
            formats.iter().filter(|f| self.is_audio_only(f)).collect();

        if audio_formats.is_empty() {
            anyhow::bail!("No audio-only formats found");
        }

        let best_audio = audio_formats
            .iter()
            .max_by_key(|f| f.bitrate.or(f.average_bitrate).unwrap_or(0))
            .ok_or_else(|| anyhow::anyhow!("Could not determine best audio format"))?;

        best_audio
            .url
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Selected format has no URL"))
    }

    fn is_audio_only(&self, format: &Format) -> bool {
        if let Some(mime) = &format.mime_type {
            if mime.starts_with("audio/") {
                return true;
            }
        }

        format.audio_quality.is_some() && format.width.is_none() && format.height.is_none()
    }

    pub async fn get_best_video_url(&self, url: &str) -> Result<String> {
        let formats = self.get_formats(url).await?;

        let video_formats: Vec<&Format> = formats
            .iter()
            .filter(|f| f.width.is_some() && f.height.is_some())
            .collect();

        if video_formats.is_empty() {
            anyhow::bail!("No video formats found");
        }

        let best_video = video_formats
            .iter()
            .max_by_key(|f| {
                let resolution = f.width.unwrap_or(0) * f.height.unwrap_or(0);
                let bitrate = f.bitrate.or(f.average_bitrate).unwrap_or(0);
                resolution as i64 * bitrate
            })
            .ok_or_else(|| anyhow::anyhow!("Could not determine best video format"))?;

        best_video
            .url
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Selected format has no URL"))
    }

    pub async fn list_formats(&self, url: &str) -> Result<()> {
        let formats = self.get_formats(url).await?;

        println!("Available formats:");
        println!(
            "{:<6} {:<40} {:<15} {:<10} {:<10}",
            "itag", "mime_type", "quality", "bitrate", "size"
        );
        println!("{}", "-".repeat(90));

        for format in formats {
            let mime = format.mime_type.as_deref().unwrap_or("unknown");
            let quality = format
                .quality_label
                .as_deref()
                .or(format.audio_quality.as_deref())
                .unwrap_or("unknown");
            let bitrate = format.bitrate.or(format.average_bitrate).unwrap_or(0);
            let size = format.content_length.as_deref().unwrap_or("?");

            println!(
                "{:<6} {:<40} {:<15} {:<10} {:<10}",
                format.itag,
                &mime[..mime.len().min(40)],
                quality,
                bitrate,
                size
            );
        }

        Ok(())
    }
}
