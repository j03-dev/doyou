use dioxus::prelude::*;

use self::dropdown_menu::DropDownMenu;
use self::music_list::MusicList;
use self::navbar::{NavBar, NavBarElement, NavBarPosition};
use self::search_bar::SearchBar;
use crate::call_api;
use crate::components::icons::SearchIcon;
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

    use_effect(move || {
        spawn(async move {
            is_loading.set(true);
            match call_api::api_suggestion().await {
                Ok(videos) => playback.playlist.set(videos.items),
                Err(err) => status_msg.set(Some(err.to_string())),
            };
            is_loading.set(false);
        });
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

        match call_api::api_search(search_query).await {
            Ok(videos) => playback.playlist.set(videos.items),
            Err(err) => status_msg.set(Some(err.to_string())),
        };
        is_loading.set(false);
        show_search.set(false);
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

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
