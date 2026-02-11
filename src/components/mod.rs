use dioxus::prelude::*;

use self::dropdown_menu::DropDownMenu;
use self::music_list::MusicList;
use self::navbar::{NavBar, NavBarElement, NavBarPosition};
use self::search_bar::SearchBar;
use crate::components::icons::SearchIcon;
use crate::config::{AppConfig, load_config, save_config};
use crate::providers::Playback;

mod alert_message;
mod dropdown_menu;
mod icons;
mod music_list;
mod music_player;
mod navbar;
mod search_bar;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    let mut is_loading = use_signal(|| false);
    let mut status_msg = use_signal(|| None::<String>);
    let mut playback = use_context_provider(|| Playback::new("audio"));
    let mut show_search = use_signal(|| false);

    let mut youtube_token = use_signal(|| None::<String>);

    use_effect(move || {
        match load_config() {
            Ok(config) => {
                if let Some(conf) = config {
                    youtube_token.set(Some(conf.youtube_token));
                }
            }
            Err(err) => status_msg.set(Some(err.to_string())),
        };

        if let Some(token) = youtube_token() {
            spawn(async move {
                is_loading.set(true);
                match yt::data_api::home(&token).await {
                    Ok(videos) => playback.playlist.set(videos.items),
                    Err(err) => status_msg.set(Some(err.to_string())),
                };
                is_loading.set(false);
            });
        } else {
            document::eval("token_form.showModal()");
        }
    });

    let search = move |evt: Event<FormData>| async move {
        evt.prevent_default();

        let search_query = evt
            .get_first("search")
            .and_then(|v| match v {
                FormValue::Text(value) => Some(value),
                _ => None,
            })
            .unwrap_or_default();

        if search_query.is_empty() {
            status_msg.set(Some("Please enter a search query.".to_string()));
            return;
        }

        status_msg.set(None);
        is_loading.set(true);

        if let Some(token) = youtube_token() {
            match yt::data_api::search(&search_query, &token).await {
                Ok(videos) => playback.playlist.set(videos.items),
                Err(err) => status_msg.set(Some(err.to_string())),
            };
        }
        is_loading.set(false);
        show_search.set(false);
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        TokenForm { youtube_token }

        NavBar {
            NavBarElement { position: NavBarPosition::Start, DropDownMenu {} }
            NavBarElement { position: NavBarPosition::Center,
                if show_search() {
                    SearchBar { on_search: search }
                } else {
                    p { class: "btn btn-ghost text-xl", "DoYou" }
                }
            }
            NavBarElement { position: NavBarPosition::End,
                button {
                    class: "btn btn-ghost btn-circle",
                    onclick: move |_| show_search.set(!show_search()),
                    SearchIcon {}
                }
            }
        }

        div { class: "m-2 pb-5",
            if let Some(message) = status_msg() {
                alert_message::AlertMessage { message }
            }
            if is_loading() {
                div { class: "flex h-screen justify-center items-center",
                    span { class: "loading loading-spinner text-secondary size-20" }
                }
            } else {
                MusicList { items: playback.playlist }
            }
        }

        div { class: "hidden",
            audio {
                id: playback.id,
                onended: move |_| playback.playback_controller(1),
                ontimeupdate: move |_| playback.update_current_time(),
                ondurationchange: move |_| playback.update_duration(),
            }
        }

        if playback.playing.read().is_some() {
            music_player::MusicPlayer {}
        }

    }
}

#[component]
fn TokenForm(mut youtube_token: Signal<Option<String>>) -> Element {
    let submit_token = move |evt: Event<FormData>| async move {
        evt.prevent_default();

        let token = evt
            .get_first("token")
            .and_then(|v| match v {
                FormValue::Text(value) => Some(value),
                _ => None,
            })
            .unwrap_or_default();

        if !token.is_empty() {
            let config = AppConfig {
                youtube_token: token,
            };
            save_config(&config).ok();
            youtube_token.set(Some(config.youtube_token));
            document::eval("token_form.close()");
        }
    };

    rsx! {
        dialog { id: "token_form", class: "modal",
            div { class: "modal-box",
                form { method: "dialog",
                    button { class: "btn btn-sm btn-circle btn-ghost absolute right-4 top-7",
                        "x"
                    }
                }

                form { onsubmit: submit_token,
                    legend { class: "fieldset-legend", "youtube data api v3 key" }

                    label { class: "label", "Token" }
                    input {
                        class: "input w-full",
                        name: "token",
                        r#type: "password",
                        placeholder: "paste your api key here (e.g. AIzaSy...)",
                    }

                    button { r#type: "submit", class: "btn btn-primary mt-5", "Save" }
                }
            }
        }
    }
}
