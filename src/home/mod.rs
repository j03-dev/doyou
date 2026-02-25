use dioxus::prelude::*;

use crate::common::components::alert_message::AlertMessage;
use crate::common::components::button::IconButton;
use crate::common::components::icons::{DoYouIcon, SearchIcon};
use crate::common::components::loading::LoadingSpinner;
use crate::common::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::common::components::text_input::TextInput;
use crate::common::context::{use_alert, use_playback, use_settings};
use crate::core::utils::get_value_from;

use self::music_list::MusicList;
use self::theme_controller::ThemeController;
use self::token_from::TokenForm;

mod music_list;
mod theme_controller;
mod token_from;

#[component]
pub fn Home() -> Element {
    let mut alert = use_alert();
    let mut playback = use_playback();
    let settings = use_settings();

    let mut is_loading = use_signal(|| false);
    let mut show_search = use_signal(|| false);

    use_effect(move || {
        settings.load();
    });

    use_effect(move || {
        if let Some(err_msg) = settings.error.read().as_ref() {
            alert.message.set(Some(err_msg.clone()));
        }
    });

    use_effect(move || {
        let token = settings.general.read().youtube_token.clone();
        spawn(async move {
            if let Some(tok) = token.as_ref() {
                if playback.playlist.is_empty() {
                    is_loading.set(true);
                    match yt::data_api::home(tok).await {
                        Ok(videos) => playback.playlist.set(videos.items),
                        Err(err) => alert.message.set(Some(err.to_string())),
                    };
                    is_loading.set(false);
                }
            } else {
                document::eval("token_form.showModal()");
            }
        });
    });

    let search = move |evt: Event<FormData>| {
        alert.message.set(None);

        let search_query = get_value_from(evt, "search");
        if search_query.is_none() {
            alert
                .message
                .set(Some("Please enter a search query.".to_string()));
            return;
        }

        spawn(async move {
            is_loading.set(true);
            if let Some(token) = settings.general.read().youtube_token.as_ref() {
                match yt::data_api::search(&search_query.unwrap(), token).await {
                    Ok(videos) => playback.playlist.set(videos.items),
                    Err(err) => alert.message.set(Some(err.to_string())),
                };
            } else {
                alert.message.set(Some("Token is not set".to_string()));
            }
            is_loading.set(false);
            show_search.set(false);
        });
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
                IconButton { on_click: move |_| show_search.set(!show_search()), SearchIcon {} }
            }
        }

        div { class: "m-2 pb-5",
            if let Some(message) = &*alert.message.read() {
                AlertMessage { message: message.clone() }
            }
            if is_loading() {
                div { class: "flex h-screen justify-center items-center",
                    LoadingSpinner { size: 20 }
                }
            } else {
                MusicList { items: playback.playlist }
            }
        }

        TokenForm {}

    }
}
