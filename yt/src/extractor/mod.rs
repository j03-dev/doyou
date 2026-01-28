use anyhow::{Context, Result};
use serde_json::json;
use types::{Format, PlayerResponse};

mod types;

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
        let api_key = "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";

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

        format.audio_quality.is_some()
    }
}
