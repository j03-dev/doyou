use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, percent_decode_str, utf8_percent_encode};
use tiny_http::{Response, Server};

const AUDIO_EXTENSIONS: [&str; 7] = ["mp3", "m4a", "flac", "ogg", "opus", "wav", "aac"];

const PATH_SET: &AsciiSet = &NON_ALPHANUMERIC.remove(b'/');

pub fn file_url(path: &str) -> Result<String, crate::core::error::Error> {
    let port = server_port()?;
    let encoded = utf8_percent_encode(path, PATH_SET).to_string();
    Ok(format!("http://127.0.0.1:{port}/{encoded}"))
}

fn server_port() -> Result<u16, crate::core::error::Error> {
    static PORT: OnceLock<u16> = OnceLock::new();
    if let Some(port) = PORT.get() {
        return Ok(*port);
    }

    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    let server = Server::from_listener(listener, None)?;
    std::thread::spawn(move || {
        for request in server.incoming_requests() {
            handle_request(request);
        }
    });
    let _ = PORT.set(port);
    Ok(port)
}

fn handle_request(request: tiny_http::Request) {
    let raw_path = request.url().split('?').next().unwrap_or("/");
    let decoded = percent_decode_str(raw_path.trim_start_matches('/'))
        .decode_utf8()
        .map(|path| PathBuf::from(path.as_ref()))
        .unwrap_or_default();

    match audio_file_response(&decoded) {
        Some(response) => {
            let _ = request.respond(response);
        }
        None => {
            let _ = request.respond(Response::from_string("Not found").with_status_code(404));
        }
    }
}

fn audio_file_response(path: &Path) -> Option<Response<std::io::Cursor<Vec<u8>>>> {
    let extension = path.extension()?.to_string_lossy().to_lowercase();
    if !AUDIO_EXTENSIONS.contains(&extension.as_str()) {
        return None;
    }

    let mime = match extension.as_str() {
        "mp3" => "audio/mpeg",
        "m4a" => "audio/mp4",
        "flac" => "audio/flac",
        "ogg" | "opus" => "audio/ogg",
        "wav" => "audio/wav",
        _ => "audio/aac",
    };

    let data = std::fs::read(path).ok()?;
    let mut response = Response::from_data(data);
    if let Ok(header) = format!("Content-Type: {mime}").parse::<tiny_http::Header>() {
        response = response.with_header(header);
    }
    Some(response)
}
