use anyhow::{Context, Result, anyhow};
use regex::Regex;
use reqwest::{
    Client,
    header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT},
};
use serde_json::json;
use std::collections::HashSet;

use types::{Format, PlayerResponse};
mod types;

const PLAYER_API: &str = "https://www.youtube.com/youtubei/v1/player";

#[derive(Debug, Clone, Copy)]
struct YoutubeClient {
    name: &'static str,
    version: &'static str,
    client_id: u32,
    platform: &'static str,
    user_agent: &'static str,
}

const CLIENTS: &[YoutubeClient] = &[
    YoutubeClient {
        name: "WEB",
        version: "2.20260708.00.00",
        client_id: 1,
        platform: "DESKTOP",
        user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
                     (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36",
    },
    YoutubeClient {
        name: "ANDROID",
        version: "21.26.364",
        client_id: 3,
        platform: "MOBILE",
        user_agent: "com.google.android.youtube/21.26.36 (Linux; U; Android 11) \
                     gzip",
    },
    YoutubeClient {
        name: "IOS",
        version: "21.26.4",
        client_id: 5,
        platform: "MOBILE",
        user_agent: "com.google.ios.youtube/21.26.4 (iPhone16,2; U; CPU iOS 18_5 \
                     like Mac OS X)",
    },
    YoutubeClient {
        name: "TVHTML5",
        version: "7.20260707.07.00",
        client_id: 7,
        platform: "TV",
        user_agent: "Mozilla/5.0 (ChromiumStylePlatform) Cobalt/Version",
    },
];

#[derive(Clone)]
pub struct YouTubeExtractor {
    client: Client,
}

impl YouTubeExtractor {
    pub fn new() -> Result<Self> {
        let mut headers = HeaderMap::new();

        headers.insert(
            ACCEPT,
            HeaderValue::from_static(
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            ),
        );

        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));

        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (X11; Linux x86_64) \
                 AppleWebKit/537.36 \
                 (KHTML, like Gecko) \
                 Chrome/140.0.0.0 Safari/537.36",
            ),
        );

        let client = Client::builder()
            .default_headers(headers)
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .context("failed to create HTTP client")?;

        Ok(Self { client })
    }

    pub async fn get_formats(&self, url: &str) -> Result<Vec<Format>> {
        let video_id = Self::extract_video_id(url)
            .ok_or_else(|| anyhow!("could not extract YouTube video ID from: {url}"))?;

        let player = self
            .fetch_player_response(&video_id)
            .await
            .with_context(|| format!("failed to fetch player response for {video_id}"))?;

        if player.playability_status.status != "OK" {
            let reason = player
                .playability_status
                .reason
                .as_deref()
                .unwrap_or("unknown reason");

            return Err(anyhow!(
                "YouTube refused playback: {} ({reason})",
                player.playability_status.status
            ));
        }

        let streaming = player
            .streaming_data
            .ok_or_else(|| anyhow!("YouTube returned no streaming data"))?;

        let mut formats = Vec::new();

        if let Some(items) = streaming.formats {
            formats.extend(items);
        }

        if let Some(items) = streaming.adaptive_formats {
            formats.extend(items);
        }

        if formats.is_empty() {
            return Err(anyhow!("YouTube returned no formats"));
        }

        formats.retain(|format| format.url.is_some() || format.signature_cipher.is_some());

        let mut seen = HashSet::new();

        formats.retain(|format| {
            if let Some(url) = &format.url {
                seen.insert(url.clone())
            } else if let Some(cipher) = &format.signature_cipher {
                seen.insert(cipher.clone())
            } else {
                false
            }
        });

        Ok(formats)
    }

    pub async fn get_best_audio_url(&self, url: &str) -> Result<String> {
        let formats = self.get_formats(url).await?;

        let mut audio_formats: Vec<&Format> = formats
            .iter()
            .filter(|format| {
                let mime = format.mime_type.as_deref().unwrap_or("");

                mime.starts_with("audio/") || format.audio_quality.is_some()
            })
            .collect();

        audio_formats.sort_by_key(|format| format.average_bitrate.or(format.bitrate).unwrap_or(0));

        let format = audio_formats
            .last()
            .ok_or_else(|| anyhow!("no audio-only format found"))?;

        if let Some(url) = &format.url {
            return Ok(url.clone());
        }

        if let Some(cipher) = &format.signature_cipher {
            let parsed = Self::parse_signature_cipher(cipher)?;

            return Ok(parsed.url);
        }

        Err(anyhow!("selected audio format has no usable URL"))
    }

    fn extract_video_id(url: &str) -> Option<String> {
        let patterns = [
            r"(?:youtube\.com/watch\?v=)([A-Za-z0-9_-]{11})",
            r"(?:youtube\.com/watch\?.*?[&?]v=)([A-Za-z0-9_-]{11})",
            r"(?:youtu\.be/)([A-Za-z0-9_-]{11})",
            r"(?:youtube\.com/embed/)([A-Za-z0-9_-]{11})",
            r"(?:youtube\.com/shorts/)([A-Za-z0-9_-]{11})",
            r"(?:youtube\.com/live/)([A-Za-z0-9_-]{11})",
            r"(?:youtube\.com/v/)([A-Za-z0-9_-]{11})",
            r"(?:youtube-nocookie\.com/embed/)([A-Za-z0-9_-]{11})",
        ];

        for pattern in patterns {
            let regex = Regex::new(pattern).ok()?;

            if let Some(captures) = regex.captures(url) {
                return captures.get(1).map(|m| m.as_str().to_owned());
            }
        }

        if Regex::new(r"^[A-Za-z0-9_-]{11}$").ok()?.is_match(url) {
            return Some(url.to_owned());
        }

        None
    }

    async fn fetch_player_response(&self, video_id: &str) -> Result<PlayerResponse> {
        let api_key = self.get_api_key().await?;

        let mut last_error = None;

        for client in CLIENTS {
            match self.fetch_with_client(video_id, &api_key, client).await {
                Ok(response) => {
                    if response.playability_status.status == "OK" {
                        return Ok(response);
                    }

                    let reason = response
                        .playability_status
                        .reason
                        .clone()
                        .unwrap_or_else(|| "unknown reason".to_string());

                    eprintln!(
                        "[youtube] client={} status={} reason={}",
                        client.name, response.playability_status.status, reason
                    );

                    last_error = Some(anyhow!(
                        "{}: {}",
                        response.playability_status.status,
                        reason
                    ));
                }

                Err(error) => {
                    eprintln!("[youtube] client={} request failed: {error:#}", client.name);

                    last_error = Some(error);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("all YouTube player clients failed")))
    }

    async fn fetch_with_client(
        &self,
        video_id: &str,
        api_key: &str,
        client: &YoutubeClient,
    ) -> Result<PlayerResponse> {
        let body = match client.name {
            "ANDROID" => json!({
                "context": {
                    "client": {
                        "clientName": client.name,
                        "clientVersion": client.version,
                        "androidSdkVersion": 30,
                        "hl": "en",
                        "gl": "US"
                    }
                },
                "videoId": video_id,
                "contentCheckOk": true,
                "racyCheckOk": true
            }),

            "IOS" => json!({
                "context": {
                    "client": {
                        "clientName": client.name,
                        "clientVersion": client.version,
                        "hl": "en",
                        "gl": "US",
                        "deviceMake": "Apple",
                        "deviceModel": "iPhone16,2",
                        "osName": "iOS",
                        "osVersion": "18.5"
                    }
                },
                "videoId": video_id,
                "contentCheckOk": true,
                "racyCheckOk": true
            }),

            "TVHTML5" => json!({
                "context": {
                    "client": {
                        "clientName": client.name,
                        "clientVersion": client.version,
                        "hl": "en",
                        "gl": "US"
                    }
                },
                "videoId": video_id,
                "contentCheckOk": true,
                "racyCheckOk": true
            }),

            _ => json!({
                "context": {
                    "client": {
                        "clientName": client.name,
                        "clientVersion": client.version,
                        "platform": client.platform,
                        "hl": "en",
                        "gl": "US"
                    }
                },
                "videoId": video_id,
                "contentCheckOk": true,
                "racyCheckOk": true
            }),
        };

        let endpoint = format!("{PLAYER_API}?key={api_key}");

        let response = self
            .client
            .post(endpoint)
            .header("X-YouTube-Client-Name", client.client_id.to_string())
            .header("X-YouTube-Client-Version", client.version)
            .header(USER_AGENT, client.user_agent)
            .json(&body)
            .send()
            .await
            .context("player request failed")?;

        let status = response.status();

        if !status.is_success() {
            let text = response.text().await.unwrap_or_default();

            return Err(anyhow!("player request returned HTTP {}: {}", status, text));
        }

        response
            .json::<PlayerResponse>()
            .await
            .context("invalid YouTube player JSON")
    }

    async fn get_api_key(&self) -> Result<String> {
        let response = self
            .client
            .get("https://www.youtube.com/")
            .send()
            .await
            .context("failed to fetch YouTube homepage")?;

        let html = response
            .text()
            .await
            .context("failed to read YouTube homepage")?;

        let patterns = [
            r#""INNERTUBE_API_KEY":"([^"]+)""#,
            r#""INNERTUBE_API_KEY":\s*"([^"]+)""#,
            r#"INNERTUBE_API_KEY['"]?\s*:\s*['"]([^'"]+)['"]"#,
        ];

        for pattern in patterns {
            let regex = Regex::new(pattern)?;

            if let Some(captures) = regex.captures(&html) {
                if let Some(key) = captures.get(1) {
                    return Ok(key.as_str().to_owned());
                }
            }
        }

        Err(anyhow!(
            "could not find INNERTUBE_API_KEY on YouTube homepage"
        ))
    }

    fn parse_signature_cipher(cipher: &str) -> Result<CipherData> {
        let mut url = None;
        let mut signature = None;
        let mut sp = "signature".to_string();

        for part in cipher.split('&') {
            let mut pieces = part.splitn(2, '=');

            let key = pieces.next().unwrap_or("");
            let value = pieces.next().unwrap_or("");

            let value = Self::percent_decode(value);

            match key {
                "url" => {
                    url = Some(value);
                }

                "s" => {
                    signature = Some(value);
                }

                "sp" => {
                    sp = value;
                }

                _ => {}
            }
        }

        let url = url.ok_or_else(|| anyhow!("signatureCipher does not contain a URL"))?;

        let signature =
            signature.ok_or_else(|| anyhow!("signatureCipher does not contain signature `s`"))?;

        // IMPORTANT:
        //
        // At this point the signature is still encrypted/obfuscated.
        // We cannot simply append `s` directly.
        //
        // yt-dlp solves this using YouTube's player JavaScript.
        //
        // We keep the parser here so the extractor is ready for
        // the signature decipher implementation.

        let separator = if url.contains('?') { '&' } else { '?' };

        let final_url = format!("{url}{separator}{sp}={signature}");

        Ok(CipherData { url: final_url })
    }

    fn percent_decode(value: &str) -> String {
        let mut output = String::with_capacity(value.len());
        let bytes = value.as_bytes();

        let mut index = 0;

        while index < bytes.len() {
            if bytes[index] == b'%' && index + 2 < bytes.len() {
                let high = bytes[index + 1];
                let low = bytes[index + 2];

                if let (Some(high), Some(low)) = (Self::hex_value(high), Self::hex_value(low)) {
                    output.push((high * 16 + low) as char);
                    index += 3;
                    continue;
                }
            }

            if bytes[index] == b'+' {
                output.push(' ');
            } else {
                output.push(bytes[index] as char);
            }

            index += 1;
        }

        output
    }

    fn hex_value(value: u8) -> Option<u8> {
        match value {
            b'0'..=b'9' => Some(value - b'0'),
            b'a'..=b'f' => Some(value - b'a' + 10),
            b'A'..=b'F' => Some(value - b'A' + 10),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct CipherData {
    url: String,
}
