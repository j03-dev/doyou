use dioxus::prelude::*;

use crate::components::{
    alert_message::AlertMessage,
    music_card::MusicCard,
    music_player::{full_music_player::FullMusicPlayer, mini_music_player::MiniMusicPlayer},
    nav_bar::NavBar,
    search_bar::SearchBar,
};
use crate::providers::Playback;
use crate::servers;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    let mut is_loading = use_signal(|| false);
    let mut status_msg = use_signal(|| None::<String>);
    let mut playback = use_context_provider(|| Playback::new("audio"));
    let mut show_full_player = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            is_loading.set(true);
            match servers::api_suggestion().await {
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

        match servers::api_search(search_query).await {
            Ok(videos) => playback.playlist.set(videos.items),
            Err(err) => status_msg.set(Some(err.to_string())),
        };
        is_loading.set(false);
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        NavBar {}
        div { class: "m-2 pb-24",
            SearchBar { on_search: search }
            if let Some(message) = status_msg() {
                AlertMessage { message }
            }
            if is_loading() {
                div { class: "flex h-screen justify-center items-center",
                    span { class: "loading loading-spinner text-secondary size-20" }
                }
            } else {
                ul { class: "list bg-base-100 rounded-box shadow-md pt-5",
                    for (index , item) in playback.playlist.read().iter().enumerate() {
                        MusicCard { item: item.clone(), index }
                    }
                }
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
        if playback.playing.read().is_some() && !show_full_player() {
            div { class: "fixed bottom-0 left-0 w-full bg-base-200 shadow-inner",
                MiniMusicPlayer { on_click: move |_| show_full_player.set(true) }
            }
        }

        if show_full_player() && playback.playing.read().is_some() {
            FullMusicPlayer { on_close: move |_| show_full_player.set(false) }
        }
    }
}
