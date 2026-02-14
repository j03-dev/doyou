use dioxus::prelude::*;

use crate::common::components::alert_message::AlertMessage;
use crate::common::components::button::{Button, IconButton};
use crate::common::components::icons::{CloseIcon, DoYouIcon, SearchIcon};
use crate::common::components::loading::LoadingSpinner;
use crate::common::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::common::components::text_input::TextInput;
use crate::core::config::AppConfig;
use crate::core::playback::Playback;
use crate::core::utils::get_value_from;

use self::music_list::MusicList;
use self::music_player::MusicPlayer;
use self::theme_controller::ThemeController;
use self::token_from::TokenForm;

mod music_list;
mod music_player;
mod theme_controller;
mod token_from;

#[component]
pub fn Home() -> Element {
    let mut playback = use_context_provider(|| Playback::new("audio"));
    let mut status_msg = use_context_provider(|| Signal::new(None::<String>));

    let mut is_loading = use_signal(|| false);
    let mut show_search = use_signal(|| false);
    let mut youtube_token = use_signal(|| None::<String>);

    use_effect(move || {
        match AppConfig::load() {
            Ok(Some(config)) => youtube_token.set(Some(config.youtube_token)),
            Err(err) => status_msg.set(Some(err.to_string())),
            _ => (),
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

        let search_query = get_value_from(evt, "search");
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
        NavBar {
            NavBarItem { position: NavBarPos::Start, ThemeController {} }
            NavBarItem { position: NavBarPos::Center,
                if show_search() {
                    TextInput {
                        on_submit: search,
                        name: "search",
                        r#type: "search",
                        placeholder: "Search",
                        SearchIcon { class: "h-[1em] opacity-50" }
                    }
                } else {
                    DoYouIcon {}
                }
            }
            NavBarItem { position: NavBarPos::End,
                IconButton { on_click: move |_| show_search.set(!show_search()), SearchIcon {} }
            }
        }

        div { class: "m-2 pb-5",
            if let Some(message) = status_msg() {
                AlertMessage { message }
            }
            if is_loading() {
                div { class: "flex h-screen justify-center items-center",
                    LoadingSpinner { size: 20 }
                }
            } else {
                MusicList { items: playback.playlist }
            }
        }

        TokeForm { youtube_token }

        div { class: "hidden",
            audio {
                id: playback.id,
                onended: move |_| playback.playback_controller(1),
                ontimeupdate: move |_| playback.update_current_time(),
                ondurationchange: move |_| playback.update_duration(),
            }
        }

        if playback.playing.read().is_some() {
            MusicPlayer {}
        }

    }
}
