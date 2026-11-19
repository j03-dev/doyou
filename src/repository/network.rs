use std::sync::Mutex;
use std::time::{Duration, Instant};

const PROBE_URL: &str = "https://www.googleapis.com/";
const PROBE_TIMEOUT: Duration = Duration::from_secs(2);
const CACHE_TTL: Duration = Duration::from_secs(10);

static CACHE: Mutex<Option<(Instant, bool)>> = Mutex::new(None);

/// Cheap, short-lived connectivity check for YouTube's API host.
///
/// The result is cached so scrolling, searching and playback don't each pay
/// for a probe. A failed probe means we skip YouTube entirely instead of
/// letting a request fail and surface a raw network error to the user.
pub async fn youtube_reachable() -> bool {
    {
        let cache = CACHE.lock().expect("network cache poisoned");
        if let Some((checked_at, reachable)) = *cache
            && checked_at.elapsed() < CACHE_TTL
        {
            return reachable;
        }
    }

    let reachable = probe().await;
    *CACHE.lock().expect("network cache poisoned") = Some((Instant::now(), reachable));
    reachable
}

async fn probe() -> bool {
    let Ok(client) = yt::client::builder().timeout(PROBE_TIMEOUT).build() else {
        return false;
    };

    // Any HTTP response proves the host is reachable; the bare endpoint
    // intentionally answers with an error status without needing an API key.
    client.get(PROBE_URL).send().await.is_ok()
}
