use dioxus::prelude::*;

use crate::core::db;
use crate::core::db::models::AppSettings;

#[derive(Clone)]
pub struct AppSettingsContext {
    pub general: Signal<AppSettings>,
    pub error: Signal<Option<String>>,
    pub is_loading: Signal<bool>,
}

impl AppSettingsContext {
    pub fn new() -> Self {
        Self {
            general: Signal::new(AppSettings::default()),
            error: Signal::new(None),
            is_loading: Signal::new(false),
        }
    }

    pub fn load(&self) {
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut general = self.general;

        is_loading.set(true);
        error.set(None);

        spawn(async move {
            match db::get_settings().await {
                Ok(settings) => {
                    general.set(settings);
                }
                Err(err) => {
                    error.set(Some(err.to_string()));
                }
            }
            is_loading.set(false);
        });
    }

    pub fn save_token(&self, token: String) {
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut general = self.general;

        is_loading.set(true);
        error.set(None);

        spawn(async move {
            match db::save_token(&token).await {
                Ok(()) => {
                    general.write().youtube_token = Some(token);
                }
                Err(err) => {
                    error.set(Some(err.to_string()));
                }
            }
            is_loading.set(false);
        });
    }
}

pub fn use_settings() -> AppSettingsContext {
    use_context::<AppSettingsContext>()
}

#[component]
pub fn AppSettingsProvider(children: Element) -> Element {
    use_context_provider(|| AppSettingsContext::new());
    rsx! {
        {children}
    }
}
