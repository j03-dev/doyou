use anyhow::Result;
use types::YouTubeResponse;

pub mod types;

const GOOGLE_API: &str = "https://www.googleapis.com/youtube/v3";

pub async fn home(key: &str) -> Result<YouTubeResponse> {
    let query = format!(
        "part=snippet&chart=mostPopular&videoCategoryId=10&regionCode=US&maxResults=10&key={key}"
    );

    let client = reqwest::Client::new();

    let response = client
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

pub async fn search(name: &str, key: &str) -> Result<YouTubeResponse> {
    let query = format!("part=snippet&q={name}&type=video&maxResults=10&key={key}",);

    let client = reqwest::Client::new();

    let response = client
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
