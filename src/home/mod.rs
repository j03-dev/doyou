use dioxus::prelude::*;

use crate::common::components::alert_message::AlertMessage;
use crate::common::components::button::IconButton;
use crate::common::components::icons::{DoYouIcon, SearchIcon};
use crate::common::components::loading::LoadingSpinner;
use crate::common::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::common::components::text_input::TextInput;
use crate::common::context::use_alert;
use crate::core::db;
use crate::core::playback::Playback;
use crate::core::utils::get_value_from;

use self::music_list::MusicList;
use self::theme_controller::ThemeController;
use self::token_from::TokenForm;

mod music_list;
mod theme_controller;
mod token_from;

#[component]
pub fn Home() -> Element {
    let alert = use_alert();
    let alert_message = alert.message;

    let playback = use_context::<Playback>();
    let mut is_loading = use_signal(|| false);
    let mut show_search = use_signal(|| false);
    let youtube_token = use_signal(|| None::<String>);

    use_effect(move || {
        let mut token_signal = youtube_token;
        let mut playlist = playback.playlist;
        let mut msg_signal = alert_message;

        spawn(async move {
            if token_signal().is_none() {
                match db::get_settings().await {
                    Ok(settings) => {
                        if !settings.youtube_token.is_empty() {
                            token_signal.set(Some(settings.youtube_token));
                        }
                    }
                    Err(err) => msg_signal.set(Some(err.to_string())),
                };
            }

            if playlist.is_empty() {
                if let Some(tok) = token_signal() {
                    is_loading.set(true);
                    match yt::data_api::home(&tok).await {
                        Ok(videos) => playlist.set(videos.items),
                        Err(err) => msg_signal.set(Some(err.to_string())),
                    };
                    is_loading.set(false);
                } else {
                    document::eval("token_form.showModal()");
                }
            }
        });
    });

    let search = move |evt: Event<FormData>| {
        let mut msg = alert_message;
        msg.set(None);

        let search_query = get_value_from(evt, "search");
        if search_query.is_none() {
            msg.set(Some("Please enter a search query.".to_string()));
            return;
        }

        let token = youtube_token();
        let mut playlist = playback.playlist;
        let mut msg_async = alert_message;

        spawn(async move {
            is_loading.set(true);
            if let Some(t) = token {
                match yt::data_api::search(&search_query.unwrap(), &t).await {
                    Ok(videos) => playlist.set(videos.items),
                    Err(err) => msg_async.set(Some(err.to_string())),
                };
            } else {
                msg_async.set(Some("Token is not none".to_string()));
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

        TokenForm { youtube_token }

    }
}
