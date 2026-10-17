use dioxus::prelude::*;
use yt::data_api::types::Item;

use crate::context::AlertProps;
use crate::context::settings::AppSettingsContext;
use crate::repository;
use crate::repository::db::models::Track;

#[component]
pub fn HomeProvider(children: Element) -> Element {
    let settings = use_context::<AppSettingsContext>();
    use_context_provider(move || HomeContext::new(settings));

    rsx! {
        {children}
    }
}

pub fn use_home() -> HomeContext {
    use_context::<HomeContext>()
}

#[derive(Clone, Copy, PartialEq)]
pub enum HomeMode {
    Feed,
    Searching,
    Results,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SourceFilter {
    All,
    Local,
    YouTube,
}

#[derive(Clone, Copy)]
pub struct HomeContext {
    pub mode: Signal<HomeMode>,
    pub feed: Signal<Vec<Item>>,
    pub results: Signal<Vec<Item>>,
    pub is_loading: Signal<bool>,
    pub error: Signal<Option<AlertProps>>,
    pub settings: AppSettingsContext,
    pub source_filter: Signal<SourceFilter>,
    pub local_tracks: Signal<Vec<Track>>,
    pub is_scanning_local: Signal<bool>,
}

impl HomeContext {
    pub fn new(settings: AppSettingsContext) -> Self {
        Self {
            mode: Signal::new(HomeMode::Feed),
            feed: Signal::new(Vec::new()),
            results: Signal::new(Vec::new()),
            is_loading: Signal::new(false),
            error: Signal::new(None),
            settings,
            source_filter: Signal::new(SourceFilter::All),
            local_tracks: Signal::new(Vec::new()),
            is_scanning_local: Signal::new(false),
        }
    }

    pub fn set_source_filter(&self, filter: SourceFilter) {
        let mut source_filter = self.source_filter;
        source_filter.set(filter);
    }

    pub fn load_feed(&self) {
        if !self.feed.read().is_empty() {
            return;
        }

        let Some(token) = self.settings.general.read().youtube_token.clone() else {
            return;
        };

        let mut feed = self.feed;
        let mut is_loading = self.is_loading;
        let mut error = self.error;

        error.set(None);
        is_loading.set(true);

        spawn(async move {
            match repository::youtube::home(&token).await {
                Ok(fetched) => feed.set(fetched),
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
            is_loading.set(false);
        });
    }

    pub fn scan_local(&self) {
        if *self.is_scanning_local.read() {
            return;
        }

        let mut local_tracks = self.local_tracks;
        let mut is_scanning = self.is_scanning_local;
        let mut error = self.error;

        is_scanning.set(true);

        spawn(async move {
            #[cfg(target_os = "android")]
            if !crate::core::platform::request_audio_permission().await {
                error.set(Some(AlertProps::warning(
                    "Storage permission is required to scan local music".to_string(),
                )));
                is_scanning.set(false);
                return;
            }

            match repository::local::scan().await {
                Ok(tracks) => local_tracks.set(tracks),
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
            is_scanning.set(false);
        });
    }

    pub fn open_search(&self) {
        let mut mode = self.mode;
        mode.set(HomeMode::Searching);
    }

    pub fn close_search(&self) {
        let mut results = self.results;
        let mut mode = self.mode;
        results.set(Vec::new());
        mode.set(HomeMode::Feed);
    }

    pub fn search(&self, query: String) {
        let mut results = self.results;
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut mode = self.mode;

        error.set(None);

        if query.is_empty() {
            error.set(Some(AlertProps::warning(
                "The input should not empty".to_string(),
            )));
            return;
        }

        let token = self.settings.general.read().youtube_token.clone();

        is_loading.set(true);
        mode.set(HomeMode::Results);

        spawn(async move {
            let mut items: Vec<Item> = Vec::new();

            match repository::local::search(&query).await {
                Ok(tracks) => items.extend(repository::tracks_to_items(&tracks)),
                Err(err) => error.set(Some(AlertProps::error(err))),
            }

            if let Some(token) = token {
                match repository::youtube::search(&query, &token).await {
                    Ok(fetched) => items.extend(fetched),
                    Err(err) => error.set(Some(AlertProps::error(err))),
                }
            }

            results.set(items);
            is_loading.set(false);
        });
    }
}
