use common::{Downloaded, Response, Videos};

static BASE_URL: &'static str = "http://localhost:5555/api/v1";

#[tauri::command]
fn search(name: &str) -> Response<Videos, String> {
    match reqwest::blocking::get(format!("{BASE_URL}/search?q={name}")) {
        Ok(response) => Response::Success(response.json().unwrap()),
        Err(err) => Response::Failed(err.to_string()),
    }
}

#[tauri::command]
fn download(video_id: &str) -> Response<Downloaded, String> {
    match reqwest::blocking::get(format!("{BASE_URL}/download?id={video_id}")) {
        Ok(response) => Response::Success(response.json().unwrap()),
        Err(err) => Response::Failed(err.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![search, download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
