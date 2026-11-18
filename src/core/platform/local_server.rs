use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, percent_decode_str, utf8_percent_encode};
use tiny_http::{Response, Server};

const PATH_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'/')
    .remove(b'.')
    .remove(b'-')
    .remove(b'_')
    .remove(b'~');

pub fn file_url(path: &str) -> Result<String, crate::core::error::Error> {
    let port = server_port()?;
    Ok(format!(
        "http://127.0.0.1:{port}/?path={}",
        encoded_path(path)
    ))
}

/// Encode the path as a query value so Unix and Windows absolute paths both
/// survive decoding unchanged (no leading-slash rewriting needed).
fn encoded_path(path: &str) -> String {
    utf8_percent_encode(path, PATH_SET).to_string()
}

fn path_from_url(url: &str) -> PathBuf {
    let Some((_, query)) = url.split_once('?') else {
        return PathBuf::new();
    };
    let Some(raw) = query.split('&').find_map(|pair| pair.strip_prefix("path=")) else {
        return PathBuf::new();
    };

    percent_decode_str(raw)
        .decode_utf8()
        .map(|path| PathBuf::from(path.as_ref()))
        .unwrap_or_default()
}

fn server_port() -> Result<u16, crate::core::error::Error> {
    static PORT: OnceLock<u16> = OnceLock::new();
    if let Some(port) = PORT.get() {
        return Ok(*port);
    }

    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    let server = Server::from_listener(listener, None).map_err(anyhow::Error::from_boxed)?;
    std::thread::spawn(move || {
        for request in server.incoming_requests() {
            handle_request(request);
        }
    });
    let _ = PORT.set(port);
    Ok(port)
}

fn handle_request(request: tiny_http::Request) {
    let decoded = path_from_url(request.url());

    match media_file_response(&decoded) {
        Some(response) => {
            let _ = request.respond(response);
        }
        None => {
            let _ = request.respond(Response::from_string("Not found").with_status_code(404));
        }
    }
}

fn media_file_response(path: &Path) -> Option<Response<std::io::Cursor<Vec<u8>>>> {
    let mime = doyou_media::MediaFormat::from_path(path)?.mime_type();

    let data = std::fs::read(path).ok()?;
    let mut response = Response::from_data(data);
    if let Ok(header) = format!("Content-Type: {mime}").parse::<tiny_http::Header>() {
        response = response.with_header(header);
    }
    Some(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_round_trip_through_url_query() {
        for path in [
            "/home/user/My Music/song.mp3",
            r"C:\Users\me\Music\song.mp3",
        ] {
            let url = format!("/?path={}", encoded_path(path));
            assert_eq!(path_from_url(&url), PathBuf::from(path));
        }
    }

    #[test]
    fn missing_path_parameter_is_empty() {
        assert_eq!(path_from_url("/?other=value"), PathBuf::new());
    }
}
