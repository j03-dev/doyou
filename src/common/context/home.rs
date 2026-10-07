use dioxus::prelude::*;
use yt::data_api::types::Item;

use crate::common::components::alert::AlertProps;
use crate::common::context::settings::AppSettingsContext;
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

#[derive(Clone, Copy)]
pub struct HomeContext {
    pub items: Signal<Vec<Item>>,
    pub is_loading: Signal<bool>,
    pub error: Signal<Option<AlertProps>>,
    pub settings: AppSettingsContext,
}

impl HomeContext {
    pub fn new(settings: AppSettingsContext) -> Self {
        Self {
            items: Signal::new(Vec::new()),
            is_loading: Signal::new(false),
            error: Signal::new(None),
            settings,
        }
    }

    pub fn load_feed(&self) {
        if !self.items.read().is_empty() {
            return;
        }

        let Some(token) = self.settings.general.read().youtube_token.clone() else {
            return;
        };

        let mut items = self.items;
        let mut error = self.error;

        error.set(None);

        spawn(async move {
            match repository::youtube::home(&token).await {
                Ok(fetched) => items.set(fetched),
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
        });
    }

    pub fn search(&self, query: String) {
        let mut items = self.items;
        let mut is_loading = self.is_loading;
        let mut error = self.error;

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

        spawn(async move {
            match repository::youtube::search(&query, &token).await {
                Ok(fetched) => items.set(fetched),
                Err(err) => error.set(Some(AlertProps::error(err))),
            }
            is_loading.set(false);
        });
    }
}
