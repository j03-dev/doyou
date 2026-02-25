use dioxus::prelude::*;

use crate::core::db;
use crate::core::db::models::AppSettings;

#[derive(Clone, Copy)]
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

    pub fn save_theme(&self, theme: String) {
        let mut is_loading = self.is_loading;
        let mut error = self.error;
        let mut general = self.general;

        is_loading.set(true);
        error.set(None);

        spawn(async move {
            match db::save_theme(&theme).await {
                Ok(()) => {
                    general.write().theme = theme;
                }
                Err(err) => error.set(Some(err.to_string())),
            }
        });
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
    let settings = use_context_provider(|| AppSettingsContext::new());

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
