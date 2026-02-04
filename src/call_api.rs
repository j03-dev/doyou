#![allow(dead_code)]
#![allow(unused_imports)]

use std::sync::LazyLock;

use dioxus::prelude::*;
use yt::data_api::{YoutubeDataApi, types::YouTubeResponse};
use yt::extractor::YouTubeExtractor;

const YOUTUBE_DATA_API: LazyLock<YoutubeDataApi> = LazyLock::new(|| YoutubeDataApi::new());
const YOUTUBE_EXTRACTOR: LazyLock<YouTubeExtractor> = LazyLock::new(|| YouTubeExtractor::new());

type Error = Box<dyn std::error::Error>;

pub async fn api_suggestion() -> Result<YouTubeResponse, Error> {
    Ok(YOUTUBE_DATA_API.home().await?)
}

pub async fn api_search(name: String) -> Result<YouTubeResponse, Error> {
    Ok(YOUTUBE_DATA_API.search(&name).await?)
}

pub async fn api_get_url(video_id: String) -> Result<String, Error> {
    Ok(YOUTUBE_EXTRACTOR
        .get_best_audio_url(&format!("https://www.youtube.com/watch?v={video_id}"))
        .await?)
}
