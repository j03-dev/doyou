use anyhow::Result;
use types::YouTubeResponse;

pub mod types;

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
