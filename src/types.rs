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
