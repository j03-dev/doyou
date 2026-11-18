use anyhow::Result;
use yt::data_api::types::Item;
use yt::extractor::YouTubeExtractor;

pub async fn home(token: &str) -> Result<Vec<Item>> {
    let response = yt::data_api::home(token).await?;

    Ok(response.items)
}

pub async fn search(query: &str, token: &str) -> Result<Vec<Item>> {
    let response = yt::data_api::search(query, token).await?;

    Ok(response.items)
}

pub async fn audio_url(video_id: &str) -> Result<String> {
    let extractor = YouTubeExtractor::new()?;

    extractor.get_best_audio_url(video_id).await
}
