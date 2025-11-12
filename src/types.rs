use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct Id {
    #[serde(rename = "videoId")]
    pub video_id: String,
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct Thumb {
    pub url: String,
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct Thumbnails {
    pub medium: Thumb,
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct Snippet {
    pub title: String,
    pub description: String,
    pub thumbnails: Thumbnails,
    #[serde(rename = "channelTitle")]
    pub channel_title: String,
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct Item {
    pub id: Id,
    pub snippet: Snippet,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Videos {
    pub items: Vec<Item>,
}

#[derive(Deserialize, Serialize)]
pub struct Download {
    pub url: String,
}
