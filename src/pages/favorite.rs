use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdPlay, LdShuffle};
use yt::data_api::types::Item;

use crate::components::alert::Alert;
use crate::components::cover::CoverCollage;
use crate::components::loading::LoadingSpinner;
use crate::components::music_row::MusicRow;
use crate::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::context::{use_favorites, use_playback, use_playlists};

#[component]
pub fn Favorite() -> Element {
    let favorites = use_favorites();
    let playback = use_playback();
    let playlists = use_playlists();

    use_effect(move || {
        favorites.fetch_all();
    });

    let items: Memo<Vec<Item>> = use_memo(move || favorites.items());
    let thumbnails = use_memo(move || {
        favorites
            .tracks
            .read()
            .iter()
            .take(4)
            .map(|t| t.thumbnail_url.clone())
            .collect::<Vec<_>>()
    });
    let count = use_memo(move || favorites.tracks.read().len());

    use_effect(move || {
        playback.set_queue(items());
    });

    let favorite_error = favorites.error;
    let playlist_error = playlists.error;
    let favorite_is_loading = favorites.is_loading;

    let play_all = move |_: Event<MouseData>| {
        let favorite_items = items();
        if favorite_items.is_empty() {
            return;
        }
        playback.set_queue(favorite_items);
        playback.start(0);
    };

    let shuffle = move |_: Event<MouseData>| {};

    rsx! {
        NavBar {
            NavBarItem { position: NavBarPos::Start,
                span {}
            }
            NavBarItem { position: NavBarPos::Center,
                p { class: "btn btn-ghost text-xl", "Favorite" }
            }
            NavBarItem { position: NavBarPos::End,
                span {}
            }
        }
        div { class: format!(
                "px-2 pt-2 mx-auto w-full max-w-6xl {}",
                if playback.playing.read().is_some() { "pb-48" } else { "pb-28" }
            ),
            if let Some(alert_props) = favorite_error() {
                Alert { ..alert_props }
            }
            if let Some(alert_props) = playlist_error() {
                Alert { ..alert_props }
            }
            div { class: "flex flex-col items-center gap-4 py-6 sm:flex-row sm:items-end",
                CoverCollage {
                    thumbnails: thumbnails(),
                    class: "w-40 rounded-box shadow-lg",
                }
                div { class: "min-w-0 text-center sm:text-left",
                    h1 { class: "truncate text-2xl font-bold", "Favorites" }
                    p { class: "text-sm text-base-content/60",
                        "{count()} "
                        if count() == 1 { "song" } else { "songs" }
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
            if favorite_is_loading() {
                div { class: "flex justify-center items-center h-40",
                    LoadingSpinner { size: 20 }
                }
            } else if items().is_empty() {
                div { class: "flex justify-center items-center h-40",
                    p { class: "text-base-content/60", "No favorites yet" }
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
