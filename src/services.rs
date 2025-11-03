use crate::types::{Downloaded, Response, Videos};

static BASE_URL: &'static str = "http://localhost:5555/api/v1";

pub async fn search(name: String) -> Response<Videos, String> {
    match reqwest::get(format!("{BASE_URL}/search?q={name}")).await {
        Ok(response) => {
            if response.status().is_server_error() || response.status().is_client_error() {
                let detail = response
                    .json::<serde_json::Value>()
                    .await
                    .unwrap_or_default();
                let message = detail
                    .get("detail")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error");
                return Response::Failed(message.to_string());
            }
            match response.json().await {
                Ok(videos) => Response::Success(videos),
                Err(e) => Response::Failed(format!("Failed to parse response: {}", e)),
            }
        }
        Err(err) => Response::Failed(err.to_string()),
    }
}

pub async fn download(video_id: String) -> Response<Downloaded, String> {
    match reqwest::get(format!("{BASE_URL}/download?id={video_id}")).await {
        Ok(response) => {
            if response.status().is_server_error() || response.status().is_client_error() {
                let detail = response
                    .json::<serde_json::Value>()
                    .await
                    .unwrap_or_default();
                let message = detail
                    .get("detail")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error");
                return Response::Failed(message.to_string());
            }
            match response.json().await {
                Ok(downloaded) => Response::Success(downloaded),
                Err(e) => Response::Failed(format!("Failed to parse response: {}", e)),
            }
        }
        Err(err) => Response::Failed(err.to_string()),
    }
}
