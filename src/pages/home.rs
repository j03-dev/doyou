use dioxus::prelude::*;

use crate::common::components::alert::{Alert, AlertLevel, AlertProps};
use crate::common::components::button::ButtonGhost;
use crate::common::components::icons::{BurgerIcon, CloseIcon, DoYouIcon, SearchIcon};
use crate::common::components::loading::LoadingSpinner;
use crate::common::components::music_list::MusicList;
use crate::common::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::common::components::text_input::TextInput;
use crate::common::context::use_settings;
use crate::core::utils::get_value_from;

#[derive(Clone, PartialEq)]
enum SearchMode {
    Home,
    Search(String),
}

#[component]
pub fn Home() -> Element {
    let settings = use_settings();

    let mut is_loading = use_signal(|| false);
    let mut show_search = use_signal(|| false);
    let mut search_mode = use_signal(|| SearchMode::Home);
    let mut alert = use_signal(|| None::<AlertProps>);

    use_effect(move || {
        if settings.general.read().youtube_token.is_none() {
            document::eval("token_form.showDialog()");
            return;
        }
    });

    let search = move |evt: Event<FormData>| {
        evt.prevent_default();
        alert.set(None);
        let search_query = get_value_from(evt, "search").unwrap_or_default();
        if search_query.is_empty() {
            alert.set(Some(AlertProps {
                level: AlertLevel::Warning,
                message: "The input should not empty".to_string(),
            }));
            return;
        }
        search_mode.set(SearchMode::Search(search_query));
    };

    let items = use_resource(move || async move {
        let mut videos_items = Vec::new();
        if let Some(key) = settings.general.read().youtube_token.clone() {
            alert.set(None);
            is_loading.set(true);
            let result = match search_mode() {
                SearchMode::Home => yt::data_api::home(&key).await,
                SearchMode::Search(q) => yt::data_api::search(&q, &key).await,
            };
            is_loading.set(false);

            match result {
                Ok(videos) => videos_items = videos.items,
                Err(err) => {
                    alert.set(Some(AlertProps {
                        level: AlertLevel::Error,
                        message: err.to_string(),
                    }));
                }
            };
        }
        videos_items
    });

    let submit_token = move |evt: Event<FormData>| {
        evt.prevent_default();
        let token = get_value_from(evt, "token");
        settings.save_token(token.unwrap());
    };

    rsx! {
        NavBar {
            NavBarItem { position: NavBarPos::Start, ThemeController {} }
            NavBarItem { position: NavBarPos::Center,
                if show_search() {
                    form { onsubmit: search,
                        TextInput {
                            name: "search",
                            r#type: "search",
                            placeholder: "Search",
                            SearchIcon { class: "h-[1em] opacity-50" }
                        }
                    }
                } else {
                    DoYouIcon {}
                }
            }
            NavBarItem { position: NavBarPos::End,
                ButtonGhost { onclick: move |_| show_search.set(!show_search()), SearchIcon {} }
            }
        }

        div { class: "m-2 pb-5",
            if let Some(alert_props) = alert() {
                Alert { ..alert_props }
            }
            if is_loading() {
                div { class: "flex h-screen justify-center items-center",
                    LoadingSpinner { size: 20 }
                }
            } else {
                if let Some(i) = items() {
                    MusicList { items: i }
                }
            }
        }

        dialog { id: "token_form", class: "modal",
            div { class: "modal-box w-96",
                form { method: "dialog",
                    button { class: "btn btn-sm absolute right-4 top-7", CloseIcon {} }
                }
                br {}
                form { onsubmit: submit_token,
                    legend { class: "fieldset-legend", "Youtube Token" }
                    TextInput {
                        name: "token",
                        r#type: "password",
                        placeholder: "paste your api key here (e.g. AIzaSy...)",
                    }
                    button { class: "btn btn-primary mt-%", r#type: "submit", "Save" }
                }
            }
        }

    }
}

#[component]
fn ThemeController() -> Element {
    let settings = use_settings();
    let themes = &["Lofi", "Black", "Night", "Forest", "Dracula"];

    rsx! {
        div { class: "dropdown",
            div {
                tabindex: 0,
                role: "button",
                class: "btn btn-ghost btn-circle",
                BurgerIcon {}
            }
            ul {
                tabindex: -1,
                class: "dropdown-content bg-base-300 rounded-box z-1 w-52 p-2 shadow-2xl",
                for theme in themes {
                    ThemeItem {
                        name: theme,
                        callback: move |theme| {
                            settings.save_theme(theme);
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn ThemeItem(name: &'static str, callback: Callback<String>) -> Element {
    rsx! {
        li {
            input {
                r#type: "radio",
                name: "theme-dropdown",
                class: "theme-controller w-full btn btn-sm btn-block btn-ghost justify-start",
                aria_label: name,
                value: name.to_lowercase(),
                onclick: move |_| callback.call(name.to_lowercase()),
            }
        }
    }
}
