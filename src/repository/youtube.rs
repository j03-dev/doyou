use yt::data_api::types::Item;
use yt::extractor::YouTubeExtractor;

pub async fn home(token: &str) -> Result<Vec<Item>, String> {
    let response = yt::data_api::home(token)
        .await
        .map_err(|err| err.to_string())?;

    Ok(response.items)
}

pub async fn search(query: &str, token: &str) -> Result<Vec<Item>, String> {
    let response = yt::data_api::search(query, token)
        .await
        .map_err(|err| err.to_string())?;

    Ok(response.items)
}

pub async fn audio_url(video_id: &str) -> Result<String, String> {
    let extractor = YouTubeExtractor::new()
        .map_err(|err| err.to_string())?;

    extractor
        .get_best_audio_url(video_id)
        .await
        .map_err(|err| err.to_string())
}
