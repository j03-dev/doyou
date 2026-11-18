use anyhow::{Context, Result, anyhow};
use regex::Regex;
use reqwest::{
    Client,
    header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT},
};
use serde_json::{Map, Value, json};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use doyou_media::AudioFormat;
use types::{Format, PlayerResponse};
mod types;

const PLAYER_API: &str = "https://www.youtube.com/youtubei/v1/player";
const ORIGIN: &str = "https://www.youtube.com";
const BROWSER_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
                                  (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36";

/// One Innertube client identity.
///
/// Every field here mirrors yt-dlp's `INNERTUBE_CLIENTS` table: a client is only
/// believable when `clientVersion`, the `userAgent` and the device/os fields all
/// describe the same build. A mismatched pair (a 7.x TV client claiming the
/// downgraded `Cobalt/Version` UA, for example) is exactly what trips YouTube's
/// bot check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct YoutubeClient {
    name: &'static str,
    version: &'static str,
    client_id: u32,
    user_agent: &'static str,
    device_make: Option<&'static str>,
    device_model: Option<&'static str>,
    os_name: Option<&'static str>,
    os_version: Option<&'static str>,
    android_sdk: Option<i32>,
}

impl YoutubeClient {
    /// The `context` block yt-dlp sends for this client.
    fn context(&self, visitor_data: &str) -> Value {
        let mut client = Map::new();

        client.insert("clientName".into(), json!(self.name));
        client.insert("clientVersion".into(), json!(self.version));
        client.insert("userAgent".into(), json!(self.user_agent));
        client.insert("hl".into(), json!("en"));
        client.insert("timeZone".into(), json!("UTC"));
        client.insert("utcOffsetMinutes".into(), json!(0));

        if let Some(make) = self.device_make {
            client.insert("deviceMake".into(), json!(make));
        }

        if let Some(model) = self.device_model {
            client.insert("deviceModel".into(), json!(model));
        }

        if let Some(name) = self.os_name {
            client.insert("osName".into(), json!(name));
        }

        if let Some(version) = self.os_version {
            client.insert("osVersion".into(), json!(version));
        }

        if let Some(sdk) = self.android_sdk {
            client.insert("androidSdkVersion".into(), json!(sdk));
        }

        if !visitor_data.is_empty() {
            client.insert("visitorData".into(), json!(visitor_data));
        }

        json!({ "client": Value::Object(client) })
    }
}

/// Tried in order. The first two are yt-dlp's JavaScript-less defaults: without a
/// JS runtime we cannot decipher `signatureCipher`, so we lead with the clients
/// that hand back plain stream URLs and keep the rest as fallbacks.
const CLIENTS: &[YoutubeClient] = &[
    YoutubeClient {
        name: "VISIONOS",
        version: "1.02",
        client_id: 101,
        user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 \
                     (KHTML, like Gecko) Version/26.0 Safari/605.1.15",
        device_make: Some("Apple"),
        device_model: Some("RealityDevice17,1"),
        os_name: Some("visionOS"),
        os_version: Some("26.5.23O471"),
        android_sdk: None,
    },
    YoutubeClient {
        name: "TVHTML5",
        version: "5.20260707",
        client_id: 7,
        user_agent: "Mozilla/5.0 (SMART-TV; Linux; Tizen 2.4.0) AppleWebKit/538.1 \
                     (KHTML, like Gecko) Version/2.4.0 TV Safari/538.1",
        device_make: Some("Samsung"),
        device_model: Some("UKS9800"),
        os_name: Some("Tizen"),
        os_version: Some("2.4.0"),
        android_sdk: None,
    },
    YoutubeClient {
        name: "ANDROID",
        version: "21.26.364",
        client_id: 3,
        user_agent: "com.google.android.youtube/21.26.364 (Linux; U; Android 11) gzip",
        device_make: None,
        device_model: None,
        os_name: Some("Android"),
        os_version: Some("11"),
        android_sdk: Some(30),
    },
    YoutubeClient {
        name: "IOS",
        version: "21.26.4",
        client_id: 5,
        user_agent: "com.google.ios.youtube/21.26.4 (iPhone16,2; U; CPU iOS 18_3_2 like Mac OS X;)",
        device_make: Some("Apple"),
        device_model: Some("iPhone16,2"),
        os_name: Some("iPhone"),
        os_version: Some("18.3.2.22D82"),
        android_sdk: None,
    },
    YoutubeClient {
        name: "WEB",
        version: "2.20260708.00.00",
        client_id: 1,
        user_agent: BROWSER_USER_AGENT,
        device_make: None,
        device_model: None,
        os_name: None,
        os_version: None,
        android_sdk: None,
    },
];

/// Values scraped from the YouTube homepage that every player request depends on.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Session {
    api_key: String,
    /// Anonymous session id. YouTube tracks it in `context.client.visitorData`
    /// and in `X-Goog-Visitor-Id`; requests without it look unfinished.
    visitor_data: String,
}

/// The exact player payload yt-dlp builds (`_generate_player_context`).
fn player_body(video_id: &str, session: &Session, client: &YoutubeClient) -> Value {
    json!({
        "context": client.context(&session.visitor_data),
        "videoId": video_id,
        "contentCheckOk": true,
        "racyCheckOk": true,
        "playbackContext": {
            "contentPlaybackContext": {
                "html5Preference": "HTML5_PREF_WANTS"
            }
        },
    })
}

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

        let client = crate::client::builder()
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

        // Only a plain URL is usable: deciphering `signatureCipher` would mean
        // running YouTube's player JavaScript, which we do not do. Responses that
        // only carry encrypted streams are rejected earlier, in
        // `fetch_player_response`, so another client can be tried instead.
        Ok(Self::direct_url_formats(formats))
    }

    /// Drops encrypted streams and keeps a single entry per plain URL.
    fn direct_url_formats(formats: Vec<Format>) -> Vec<Format> {
        let mut seen = HashSet::new();

        formats
            .into_iter()
            .filter(|format| format.url.is_some())
            .filter(|format| {
                format
                    .url
                    .as_ref()
                    .is_some_and(|url| seen.insert(url.clone()))
            })
            .collect()
    }

    pub async fn get_best_audio_url(&self, url: &str) -> Result<String> {
        let format = self.best_audio_format(url).await?;
        Self::format_url(&format)
    }

    /// Downloads the best audio-only stream into `directory` and returns the file path.
    pub async fn download_audio(&self, url: &str, directory: &Path, stem: &str) -> Result<PathBuf> {
        let format = self.best_audio_format(url).await?;
        let stream_url = Self::format_url(&format)?;
        let audio_format = format
            .mime_type
            .as_deref()
            .and_then(AudioFormat::from_mime)
            .unwrap_or(AudioFormat::M4a);

        let response = self
            .client
            .get(&stream_url)
            .header(USER_AGENT, BROWSER_USER_AGENT)
            .send()
            .await
            .context("failed to download audio stream")?
            .error_for_status()
            .context("audio stream request failed")?;

        let bytes = response
            .bytes()
            .await
            .context("failed to read audio stream")?;

        let target = directory.join(format!("{stem}.{}", audio_format.as_str()));
        std::fs::write(&target, &bytes)
            .with_context(|| format!("failed to write {}", target.display()))?;

        Ok(target)
    }

    async fn best_audio_format(&self, url: &str) -> Result<Format> {
        let formats = self.get_formats(url).await?;

        let mut audio_formats: Vec<&Format> = formats
            .iter()
            .filter(|format| {
                let mime = format.mime_type.as_deref().unwrap_or("");

                mime.starts_with("audio/") || format.audio_quality.is_some()
            })
            .collect();

        audio_formats.sort_by_key(|format| format.average_bitrate.or(format.bitrate).unwrap_or(0));

        let best = audio_formats
            .last()
            .ok_or_else(|| anyhow!("no audio-only format found"))?;

        Ok((*best).clone())
    }

    fn format_url(format: &Format) -> Result<String> {
        format
            .url
            .clone()
            .ok_or_else(|| anyhow!("selected audio format has no directly playable URL"))
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
        let session = self.fetch_session().await?;

        let mut last_error = None;
        let mut sign_in_required = 0usize;
        let mut attempts = 0usize;

        for client in CLIENTS {
            attempts += 1;

            match self.fetch_with_client(video_id, &session, client).await {
                Ok(response) => {
                    if response.playability_status.status == "OK" {
                        if Self::has_direct_audio(&response) {
                            return Ok(response);
                        }

                        // The streams are all `signatureCipher`, so they are
                        // unusable without a JS runtime. Move on to the next
                        // client instead of handing back URLs that would 403.
                        eprintln!(
                            "[youtube] client={} returned no directly playable stream; skipping",
                            client.name
                        );

                        last_error = Some(anyhow!(
                            "{} returned only encrypted stream URLs",
                            client.name
                        ));

                        continue;
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

                    if response.playability_status.status == "LOGIN_REQUIRED" {
                        sign_in_required += 1;
                    }

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

        if attempts > 0 && sign_in_required == attempts {
            return Err(anyhow!(
                "YouTube wants a sign-in to confirm this request is not a bot; \
                 no client can serve this video without an account session"
            ));
        }

        Err(last_error.unwrap_or_else(|| anyhow!("all YouTube player clients failed")))
    }

    /// True when at least one audio stream came back with a plain URL.
    fn has_direct_audio(player: &PlayerResponse) -> bool {
        let Some(streaming) = &player.streaming_data else {
            return false;
        };

        streaming
            .formats
            .iter()
            .flatten()
            .chain(streaming.adaptive_formats.iter().flatten())
            .any(|format| Self::is_audio(format) && format.url.is_some())
    }

    fn is_audio(format: &Format) -> bool {
        format
            .mime_type
            .as_deref()
            .is_some_and(|mime| mime.starts_with("audio/"))
            || format.audio_quality.is_some()
    }

    async fn fetch_with_client(
        &self,
        video_id: &str,
        session: &Session,
        client: &YoutubeClient,
    ) -> Result<PlayerResponse> {
        let body = player_body(video_id, session, client);
        let endpoint = format!("{PLAYER_API}?key={}&prettyPrint=false", session.api_key);

        let mut request = self
            .client
            .post(endpoint)
            .header("X-YouTube-Client-Name", client.client_id.to_string())
            .header("X-YouTube-Client-Version", client.version)
            .header(USER_AGENT, client.user_agent)
            .header("Origin", ORIGIN)
            .json(&body);

        if !session.visitor_data.is_empty() {
            request = request.header("X-Goog-Visitor-Id", &session.visitor_data);
        }

        let response = request.send().await.context("player request failed")?;

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

    /// Scrapes the API key and the anonymous `visitorData` in a single pass.
    async fn fetch_session(&self) -> Result<Session> {
        let response = self
            .client
            .get(ORIGIN)
            .send()
            .await
            .context("failed to fetch YouTube homepage")?;

        let html = response
            .text()
            .await
            .context("failed to read YouTube homepage")?;

        let api_key = Self::capture(&html, &[r#""INNERTUBE_API_KEY":"([^"]+)""#])
            .ok_or_else(|| anyhow!("could not find INNERTUBE_API_KEY on YouTube homepage"))?;

        let visitor_data = Self::capture(
            &html,
            &[
                r#""visitorData":"([^"]+)""#,
                r#""VISITOR_DATA":\s*"([^"]+)""#,
            ],
        )
        .unwrap_or_default();

        Ok(Session {
            api_key,
            visitor_data,
        })
    }

    fn capture(html: &str, patterns: &[&str]) -> Option<String> {
        patterns.iter().find_map(|pattern| {
            let regex = Regex::new(pattern).ok()?;

            regex
                .captures(html)
                .and_then(|captures| captures.get(1))
                .map(|value| value.as_str().to_owned())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::{PlayabilityStatus, StreamingData};

    fn audio_format(url: Option<&str>, cipher: Option<&str>) -> Format {
        Format {
            url: url.map(str::to_owned),
            signature_cipher: cipher.map(str::to_owned),
            mime_type: Some("audio/webm; codecs=\"opus\"".to_string()),
            bitrate: Some(160_000),
            audio_quality: Some("AUDIO_QUALITY_MEDIUM".to_string()),
            content_length: None,
            average_bitrate: Some(160_000),
        }
    }

    fn player_with(formats: Vec<Format>) -> PlayerResponse {
        PlayerResponse {
            streaming_data: Some(StreamingData {
                formats: None,
                adaptive_formats: Some(formats),
            }),
            playability_status: PlayabilityStatus {
                status: "OK".to_string(),
                reason: None,
            },
        }
    }

    #[test]
    fn every_client_declares_a_consistent_identity() {
        for client in CLIENTS {
            let context = client.context("");
            let declared = context
                .get("client")
                .and_then(Value::as_object)
                .expect("context must contain a client object");

            assert_eq!(
                declared.get("clientName").and_then(Value::as_str),
                Some(client.name)
            );
            assert_eq!(
                declared.get("clientVersion").and_then(Value::as_str),
                Some(client.version)
            );
            assert_eq!(
                declared.get("userAgent").and_then(Value::as_str),
                Some(client.user_agent)
            );
            assert_eq!(declared.get("hl").and_then(Value::as_str), Some("en"));
        }
    }

    /// A user agent that disagrees with `clientVersion` is the classic bot-check
    /// trigger: this is what the old ANDROID (`21.26.36` vs `21.26.364`) and TV
    /// (`7.x` vs `Cobalt/Version`) entries got wrong.
    #[test]
    fn mobile_user_agents_match_their_client_version() {
        for client in CLIENTS {
            if matches!(client.name, "ANDROID" | "IOS") {
                assert!(
                    client.user_agent.contains(client.version),
                    "{} user agent {:?} does not contain version {}",
                    client.name,
                    client.user_agent,
                    client.version
                );
            }
        }
    }

    #[test]
    fn player_body_carries_visitor_data_and_html5_preference() {
        let session = Session {
            api_key: "key".to_string(),
            visitor_data: "visitor".to_string(),
        };

        let body = player_body("dQw4w9WgXcQ", &session, &CLIENTS[0]);

        assert_eq!(
            body.get("videoId").and_then(Value::as_str),
            Some("dQw4w9WgXcQ")
        );
        assert_eq!(body.get("contentCheckOk"), Some(&Value::Bool(true)));
        assert_eq!(
            body.pointer("/playbackContext/contentPlaybackContext/html5Preference"),
            Some(&Value::String("HTML5_PREF_WANTS".to_string()))
        );
        assert_eq!(
            body.pointer("/context/client/visitorData")
                .and_then(Value::as_str),
            Some("visitor")
        );
    }

    #[test]
    fn player_body_omits_visitor_data_when_unknown() {
        let body = player_body("dQw4w9WgXcQ", &Session::default(), &CLIENTS[0]);

        assert!(body.pointer("/context/client/visitorData").is_none());
    }

    #[test]
    fn encrypted_streams_are_not_treated_as_playable() {
        let encrypted = player_with(vec![audio_format(
            None,
            Some("s=abc&url=https%3A%2F%2Fexample.invalid"),
        )]);

        assert!(!YouTubeExtractor::has_direct_audio(&encrypted));

        let direct = player_with(vec![audio_format(Some("https://example.invalid/a"), None)]);

        assert!(YouTubeExtractor::has_direct_audio(&direct));
    }

    #[test]
    fn only_plain_urls_are_kept() {
        let kept = YouTubeExtractor::direct_url_formats(vec![
            audio_format(Some("https://example.invalid/a"), None),
            audio_format(None, Some("s=abc&url=https%3A%2F%2Fexample.invalid")),
            audio_format(Some("https://example.invalid/a"), None),
            audio_format(Some("https://example.invalid/b"), None),
        ]);

        let urls: Vec<&str> = kept
            .iter()
            .filter_map(|format| format.url.as_deref())
            .collect();

        assert_eq!(
            urls,
            ["https://example.invalid/a", "https://example.invalid/b"]
        );
    }

    #[test]
    fn session_values_are_scraped_from_the_homepage() {
        let html = r#"ytcfg.set({"INNERTUBE_API_KEY":"AIzaSyTEST","VISITOR_DATA":"Cgt0ZXN0"});"#;
        let api_key = YouTubeExtractor::capture(html, &[r#""INNERTUBE_API_KEY":"([^"]+)""#]);
        let visitor = YouTubeExtractor::capture(
            html,
            &[
                r#""visitorData":"([^"]+)""#,
                r#""VISITOR_DATA":\s*"([^"]+)""#,
            ],
        );

        assert_eq!(api_key.as_deref(), Some("AIzaSyTEST"));
        assert_eq!(visitor.as_deref(), Some("Cgt0ZXN0"));
    }

    /// Live smoke test against YouTube. Ignored by default because it needs the
    /// network; run it with:
    ///
    /// ```text
    /// cargo test -p yt -- --ignored --nocapture
    /// ```
    #[tokio::test]
    #[ignore = "hits youtube.com"]
    async fn live_request_returns_a_directly_playable_audio_stream() {
        let extractor = YouTubeExtractor::new().expect("http client");

        let formats = extractor
            .get_formats("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
            .await
            .expect("player request should succeed");

        assert!(
            formats
                .iter()
                .any(|format| YouTubeExtractor::is_audio(format) && format.url.is_some()),
            "expected at least one audio format with a plain URL"
        );
    }
}
