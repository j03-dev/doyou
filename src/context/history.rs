use dioxus::prelude::*;

use crate::context::AlertProps;
use crate::repository;
use crate::repository::db::models::Track;

const HISTORY_LIMIT: usize = 5;

#[component]
pub fn HistoryProvider(children: Element) -> Element {
    use_context_provider(HistoryContext::new);
    rsx! {
        {children}
    }
}

pub fn use_history() -> HistoryContext {
    use_context::<HistoryContext>()
}

#[derive(Clone, Copy, PartialEq)]
pub struct HistoryContext {
    pub recent: Signal<Vec<Track>>,
    pub most: Signal<Vec<Track>>,
    pub is_loading: Signal<bool>,
    pub error: Signal<Option<AlertProps>>,
}

impl HistoryContext {
    pub fn new() -> Self {
        Self {
            recent: Signal::new(Vec::new()),
            most: Signal::new(Vec::new()),
            is_loading: Signal::new(false),
            error: Signal::new(None),
        }
    }

    pub fn fetch_all(&self) {
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut recent = self.recent;
        let mut most = self.most;

        is_loading.set(true);
        error.set(None);

        spawn(async move {
            match repository::history::recently_played(HISTORY_LIMIT).await {
                Ok(fetched) => recent.set(fetched),
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
            match repository::history::most_played(HISTORY_LIMIT).await {
                Ok(fetched) => most.set(fetched),
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
            is_loading.set(false);
        });
    }

    pub fn recent_items(&self) -> Vec<yt::data_api::types::Item> {
        repository::tracks_to_items(&self.recent.read())
    }

    pub fn most_items(&self) -> Vec<yt::data_api::types::Item> {
        repository::tracks_to_items(&self.most.read())
    }
}
