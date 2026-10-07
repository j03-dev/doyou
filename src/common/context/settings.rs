use dioxus::prelude::*;

use crate::common::components::alert::AlertProps;
use crate::core::db::models::AppSettings;
use crate::repository;

#[component]
pub fn AppSettingsProvider(children: Element) -> Element {
    let settings = use_context_provider(AppSettingsContext::new);

    use_effect(move || {
        settings.load();
    });

    use_effect(move || {
        document::eval(&format!(
            "document.documentElement.setAttribute('data-theme', '{}')",
            settings.general.read().theme,
        ));
    });

    rsx! {
        {children}
    }
}

pub fn use_settings() -> AppSettingsContext {
    use_context::<AppSettingsContext>()
}

#[derive(Clone, Copy)]
pub struct AppSettingsContext {
    pub general: Signal<AppSettings>,
    pub error: Signal<Option<AlertProps>>,
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

    pub fn save_theme(&self, theme: String) {
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut general = self.general;

        is_loading.set(true);
        error.set(None);

        spawn(async move {
            match repository::settings::save_theme(&theme).await {
                Ok(()) => general.write().theme = theme,
                Err(err) => error.set(Some(AlertProps::error(err))),
            };
            is_loading.set(false);
        });
    }

    pub fn load(&self) {
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut general = self.general;

        is_loading.set(true);
        error.set(None);

        spawn(async move {
            match repository::settings::load().await {
                Ok(settings) => general.set(settings),
                Err(err) => error.set(Some(AlertProps::error(err))),
            };
            is_loading.set(false);
        });
    }

    pub fn save_token(&self, token: String) {
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut general = self.general;

        error.set(None);

        if token.is_empty() {
            error.set(Some(AlertProps::warning(
                "The token should not empty".to_string(),
            )));
            return;
        }

        is_loading.set(true);

        spawn(async move {
            match repository::settings::save_token(&token).await {
                Ok(()) => general.write().youtube_token = Some(token),
                Err(err) => error.set(Some(AlertProps::error(err))),
            };
            is_loading.set(false);
        });
    }
}
