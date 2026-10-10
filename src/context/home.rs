use dioxus::prelude::*;
use yt::data_api::types::Item;

use crate::context::AlertProps;
use crate::context::settings::AppSettingsContext;
use crate::repository;

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

#[derive(Clone, Copy)]
pub struct HomeContext {
    pub mode: Signal<HomeMode>,
    pub feed: Signal<Vec<Item>>,
    pub results: Signal<Vec<Item>>,
    pub is_loading: Signal<bool>,
    pub error: Signal<Option<AlertProps>>,
    pub settings: AppSettingsContext,
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
        }
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

        let Some(token) = self.settings.general.read().youtube_token.clone() else {
            error.set(Some(AlertProps::info(
                "Pls setup you token first".to_string(),
            )));
            return;
        };

        is_loading.set(true);
        mode.set(HomeMode::Results);

        spawn(async move {
            match repository::youtube::search(&query, &token).await {
                Ok(fetched) => results.set(fetched),
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
            is_loading.set(false);
        });
    }
}
