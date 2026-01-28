#![allow(dead_code)]
#![allow(unused_imports)]

use dioxus::prelude::*;
use std::sync::LazyLock;

use yt::data_api::{YoutubeDataApi, types::YouTubeResponse};
use yt::extractor::YouTubeExtractor;

const YOUTUBE_DATA_API: LazyLock<YoutubeDataApi> = LazyLock::new(|| YoutubeDataApi::new());
const YOUTUBE_EXRACTOR: LazyLock<YouTubeExtractor> = LazyLock::new(|| YouTubeExtractor::new());

#[allow(dead_code)]
const GOOGLE_API: &str = "https://www.googleapis.com/youtube/v3";

#[get("/api/suggestion")]
pub async fn api_suggestion() -> Result<YouTubeResponse, ServerFnError> {
    YOUTUBE_DATA_API
        .home()
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))
}

#[get("/api/search?name")]
pub async fn api_search(name: String) -> Result<YouTubeResponse, ServerFnError> {
    YOUTUBE_DATA_API
        .search(&name)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))
}

#[get("/api/url?video_id")]
pub async fn api_get_url(video_id: String) -> Result<String, ServerFnError> {
    YOUTUBE_EXRACTOR
        .get_best_audio_url(&format!("https://www.youtube.com/watch?v={video_id}"))
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))
}
