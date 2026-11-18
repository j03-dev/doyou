use dioxus::prelude::*;
use yt::data_api::types::Item;

use crate::context::AlertProps;
use crate::repository;
use crate::repository::TrackSource;
use crate::repository::db::models::Track;

#[component]
pub fn DownloadsProvider(children: Element) -> Element {
    use_context_provider(DownloadsContext::new);
    rsx! {
        {children}
    }
}

pub fn use_downloads() -> DownloadsContext {
    use_context::<DownloadsContext>()
}

#[derive(Clone, Copy, PartialEq)]
pub struct DownloadsContext {
    pub tracks: Signal<Vec<Track>>,
    pub pending: Signal<Vec<String>>,
    pub is_loading: Signal<bool>,
    pub error: Signal<Option<AlertProps>>,
}

impl DownloadsContext {
    pub fn new() -> Self {
        Self {
            tracks: Signal::new(Vec::new()),
            pending: Signal::new(Vec::new()),
            is_loading: Signal::new(false),
            error: Signal::new(None),
        }
    }

    pub fn fetch_all(&self) {
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut tracks = self.tracks;

        is_loading.set(true);
        error.set(None);

        spawn(async move {
            match repository::downloads::list().await {
                Ok(fetched) => tracks.set(fetched),
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
            is_loading.set(false);
        });
    }

    pub fn download(&self, item: &Item) {
        let Some(video_id) = item.id.as_string() else {
            return;
        };
        if self.is_pending(&video_id) || self.is_downloaded(&video_id) {
            return;
        }

        let item = item.clone();
        let mut pending = self.pending;
        let mut tracks = self.tracks;
        let mut error = self.error;

        pending.write().push(video_id.clone());
        error.set(None);

        spawn(async move {
            match repository::downloads::download(&item).await {
                Ok(track) => {
                    let mut saved = tracks.write();
                    saved.retain(|saved| saved.id != track.id);
                    saved.push(track);
                }
                Err(err) => error.set(Some(AlertProps::error(format!("Download failed: {err}")))),
            }
            pending.write().retain(|id| id != &video_id);
        });
    }

    pub fn remove(&self, track_id: &str) {
        let track_id = track_id.to_string();
        let mut tracks = self.tracks;
        let mut error = self.error;

        spawn(async move {
            match repository::downloads::remove(&track_id).await {
                Ok(()) => tracks.write().retain(|track| track.id != track_id),
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
        });
    }

    pub fn is_pending(&self, video_id: &str) -> bool {
        self.pending.read().iter().any(|id| id == video_id)
    }

    pub fn is_downloaded(&self, video_id: &str) -> bool {
        let id = TrackSource::Downloaded.id(video_id);
        self.tracks.read().iter().any(|track| track.id == id)
    }

    pub fn items(&self) -> Vec<Item> {
        repository::tracks_to_items(&self.tracks.read())
    }
}
