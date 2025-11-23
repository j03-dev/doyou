use crate::types::YouTubeResponse;
use dioxus::prelude::*;

const GOOGLE_API: &str = "https://www.googleapis.com/youtube/v3";

#[get("/api/suggestion")]
pub async fn api_suggestion() -> Result<YouTubeResponse, ServerFnError> {
    let key = match std::env::var("GOOGLE_API_KEY") {
        Ok(k) => k,
        Err(err) => return Err(ServerFnError::new(err.to_string())),
    };

    let query = format!(
        "part=snippet&chart=mostPopular&videoCategoryId=10&regionCode=US&maxResults=10&key={key}"
    );

    match reqwest::get(format!("{GOOGLE_API}/videos?{query}")).await {
        Ok(response) => {
            if response.status().is_client_error() || response.status().is_server_error() {
                return Err(ServerFnError::new(response.text().await.unwrap()));
            }
            return Ok(response
                .json()
                .await
                .map_err(|err| ServerFnError::new(err.to_string()))?);
        }
        Err(err) => Err(ServerFnError::new(err.to_string())),
    }
}

#[get("/api/search?name")]
pub async fn api_search(name: String) -> Result<YouTubeResponse, ServerFnError> {
    let key = match std::env::var("GOOGLE_API_KEY") {
        Ok(k) => k,
        Err(err) => return Err(ServerFnError::new(err.to_string())),
    };

    let query =
        format!("part=snippet&q={name}&type=video&videoCategoryId=10&maxResults=10&key={key}");

    match reqwest::get(format!("{GOOGLE_API}/search?{query}")).await {
        Ok(response) => {
            if response.status().is_client_error() || response.status().is_server_error() {
                return Err(ServerFnError::new(response.text().await.unwrap()));
            }
            return Ok(response
                .json()
                .await
                .map_err(|err| ServerFnError::new(err.to_string()))?);
        }
        Err(err) => Err(ServerFnError::new(err.to_string())),
    }
}

#[get("/api/url?video_id")]
pub async fn api_get_url(video_id: String) -> Result<String, ServerFnError> {
    let url = format!("https://www.youtube.com/watch?v={video_id}");
    match std::process::Command::new("yt-dlp")
        .args(&["-f", "bestaudio", "--get-url", "--no-playlist", &url])
        .output()
    {
        Ok(output) => {
            let audio_url = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(audio_url)
        }
        Err(err) => Err(ServerFnError::new(err.to_string())),
    }
}
