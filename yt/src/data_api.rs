pub mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
    pub struct IdObj {
        #[serde(rename = "videoId")]
        pub video_id: Option<String>,
        #[serde(rename = "channelId")]
        pub channel_id: Option<String>,
        #[serde(rename = "playlistId")]
        pub playlist_id: Option<String>,
        pub kind: Option<String>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
    #[serde(untagged)]
    pub enum VideoId {
        Literal(String),
        Object(IdObj),
    }

    impl VideoId {
        pub fn as_string(&self) -> Option<String> {
            match self {
                VideoId::Literal(s) => Some(s.clone()),
                VideoId::Object(obj) => obj
                    .video_id
                    .clone()
                    .or(obj.channel_id.clone())
                    .or(obj.playlist_id.clone()),
            }
        }
    }

    #[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
    pub struct Thumb {
        pub url: String,
        pub width: Option<u32>,
        pub height: Option<u32>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
    pub struct Thumbnails {
        pub default: Option<Thumb>,
        pub medium: Option<Thumb>,
        pub high: Option<Thumb>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
    pub struct Snippet {
        pub title: String,
        pub description: String,
        #[serde(rename = "publishedAt")]
        pub published_at: Option<String>,
        #[serde(rename = "channelTitle")]
        pub channel_title: Option<String>,
        pub thumbnails: Option<Thumbnails>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
    pub struct Item {
        pub id: VideoId,
        pub snippet: Snippet,
    }

    #[derive(Deserialize, Serialize, Clone, Debug)]
    pub struct YouTubeResponse {
        pub kind: String,
        pub items: Vec<Item>,
        #[serde(rename = "nextPageToken")]
        pub next_page_token: Option<String>,
    }
}

use self::types::YouTubeResponse;
use anyhow::Result;

const GOOGLE_API: &str = "https://www.googleapis.com/youtube/v3";

pub struct YoutubeDataApi {
    client: reqwest::Client,
    api_key: String,
}

impl YoutubeDataApi {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY is not set on .env"),
        }
    }

    pub async fn home(&self) -> Result<YouTubeResponse> {
        let query = format!(
            "part=snippet&chart=mostPopular&videoCategoryId=10&regionCode=US&maxResults=10&key={key}",
            key = self.api_key
        );

        let response = self
            .client
            .get(format!("{GOOGLE_API}/videos?{query}"))
            .send()
            .await
            .map_err(|err| anyhow::anyhow!(err.to_string()))?;

        if !response.status().is_success() {
            anyhow::bail!(response.text().await.unwrap());
        }

        response
            .json()
            .await
            .map_err(|err| anyhow::anyhow!(err.to_string()))
    }

    pub async fn search(&self, name: &str) -> Result<YouTubeResponse> {
        let query = format!(
            "part=snippet&q={name}&type=video&maxResults=10&key={key}",
            key = self.api_key
        );

        let response = self
            .client
            .get(format!("{GOOGLE_API}/search?{query}"))
            .send()
            .await
            .map_err(|err| anyhow::anyhow!(err.to_string()))?;

        if !response.status().is_success() {
            anyhow::bail!(response.text().await.unwrap());
        }

        response
            .json()
            .await
            .map_err(|err| anyhow::anyhow!(err.to_string()))
    }
}
