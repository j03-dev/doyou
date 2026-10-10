use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdArrowLeft, LdPlay, LdShuffle};
use yt::data_api::types::Item;

use crate::components::alert::Alert;
use crate::components::cover::CoverCollage;
use crate::components::loading::LoadingSpinner;
use crate::components::music_row::MusicRow;
use crate::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::context::{use_playback, use_playlists};
use crate::repository::tracks_to_items;

#[component]
pub fn Playlist(id: i32) -> Element {
    let playlists = use_playlists();
    let playback = use_playback();
    let nav = use_navigator();

    use_effect(move || {
        playlists.load_detail(id);
    });

    let detail = playlists.detail;
    let error = playlists.error;
    let is_loading = playlists.detail_is_loading;

    let items: Memo<Vec<Item>> = use_memo(move || {
        detail
            .read()
            .as_ref()
            .map(|entry| tracks_to_items(&entry.tracks))
            .unwrap_or_default()
    });

    use_effect(move || {
        playback.set_queue(items());
    });

    let play_all = move |_: Event<MouseData>| {
        let list = items();
        if list.is_empty() {
            return;
        }
        playback.set_queue(list);
        playback.start(0);
    };

    let shuffle = move |_: Event<MouseData>| {};

    rsx! {
        NavBar {
            NavBarItem { position: NavBarPos::Start,
                button {
                    class: "btn btn-ghost btn-circle",
                    onclick: move |_| nav.go_back(),
                    Icon { icon: LdArrowLeft }
                }
            }
            NavBarItem { position: NavBarPos::Center,
                p { class: "btn btn-ghost text-xl", "Playlist" }
            }
            NavBarItem { position: NavBarPos::End,
                span {}
            }
        }
        div { class: format!(
                "px-2 pt-2 mx-auto w-full max-w-6xl {}",
                if playback.playing.read().is_some() { "pb-48" } else { "pb-28" }
            ),
            if let Some(alert_props) = error() {
                Alert { ..alert_props }
            }
            if is_loading() {
                div { class: "flex h-40 justify-center items-center",
                    LoadingSpinner { size: 20 }
                }
            } else if let Some(entry) = detail() {
                div { class: "flex flex-col items-center gap-4 py-6 sm:flex-row sm:items-end",
                    CoverCollage {
                        thumbnails: entry.thumbnails(),
                        class: "w-40 rounded-box shadow-lg",
                    }
                    div { class: "min-w-0 text-center sm:text-left",
                        h1 { class: "truncate text-2xl font-bold", "{entry.playlist.name}" }
                        p { class: "text-sm text-base-content/60",
                            "{entry.tracks.len()} "
                            if entry.tracks.len() == 1 { "song" } else { "songs" }
                        }
                    }
                }
                div { class: "flex justify-center items-center gap-3 w-full max-w-3xl mx-auto px-4 mt-2",
                    button {
                        class: "btn btn-lg btn-primary h-16 w-1/2",
                        onclick: play_all,
                        Icon { icon: LdPlay }
                        span { "Play All" }
                    }
                    button {
                        class: "btn btn-lg btn-neutral h-16 w-1/4 rounded-full",
                        onclick: shuffle,
                        Icon { icon: LdShuffle }
                    }
                }
                if items().is_empty() {
                    div { class: "flex justify-center items-center h-40",
                        p { class: "text-base-content/60", "No songs yet" }
                    }
                } else {
                    ul { class: "list bg-base-100 rounded-box shadow-md mt-3",
                        for (index, item) in items().iter().enumerate() {
                            MusicRow { item: item.clone(), index }
                        }
                    }
                }
            }
        }
    }
}
