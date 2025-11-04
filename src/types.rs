use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Id {
    #[serde(rename = "videoId")]
    pub video_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Thumb {
    pub url: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Thumbnails {
    pub medium: Thumb,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Snippet {
    pub title: String,
    pub description: String,
    pub thumbnails: Thumbnails,
    #[serde(rename = "channelTitle")]
    pub channel_title: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Item {
    pub id: Id,
    pub snippet: Snippet,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Videos {
    pub items: Vec<Item>,
}

#[derive(Deserialize, Serialize)]
pub struct Downloaded {
    pub video_id: String,
}

#[derive(Deserialize, Serialize)]
pub enum Response<T, E> {
    Success(T),
    Failed(E),
}
