use commons::{Download, Response, Videos};

static GOOGLE_API: &str = "https://www.googleapis.com/youtube/v3";

#[tauri::command]
async fn search(name: String) -> Response<Videos, String> {
    let key = std::env::var("GOOGLE_API_KEY").unwrap();
    let query = format!("search?part=snippet&q={name}&type=video&maxResults=10&key={key}");
    match reqwest::get(format!("{GOOGLE_API}/{query}")).await {
        Ok(response) => {
            if response.status().is_client_error() || response.status().is_server_error() {
                return Response::Failed(response.text().await.unwrap());
            }
            return Response::Success(response.json().await.unwrap());
        }
        Err(err) => Response::Failed(err.to_string()),
    }
}

#[tauri::command]
fn download(video_id: String) -> Response<Download, String> {
    let url = format!("https://www.youtube.com/watch?v={video_id}");
    match std::process::Command::new("yt-dlp")
        .args(&["-f", "bestaudio", "--get-url", "--no-playlist", &url])
        .output()
    {
        Ok(output) => {
            let audio_url = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Response::Success(Download { url: audio_url })
        }
        Err(err) => Response::Failed(err.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![download, search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
