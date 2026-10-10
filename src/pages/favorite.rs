use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdHeart, LdListMusic, LdPlay, LdShuffle};
use yt::data_api::types::Item;

use crate::components::alert::Alert;
use crate::components::collection_hero::CollectionHero;
use crate::components::empty_state::EmptyState;
use crate::components::loading::LoadingSpinner;
use crate::components::music_row::MusicRow;
use crate::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::components::section_header::SectionHeader;
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

    let shuffle = move |_: Event<MouseData>| {
        playback.start_shuffled(items());
    };

    rsx! {
        NavBar {
            NavBarItem { position: NavBarPos::Start,
                span {}
            }
            NavBarItem { position: NavBarPos::Center,
                p { class: "btn btn-ghost text-xl", "Favorites" }
            }
            NavBarItem { position: NavBarPos::End,
                span {}
            }
        }
        div { class: format!(
                "px-4 pt-4 mx-auto w-full max-w-4xl sm:pt-6 {}",
                if playback.playing.read().is_some() { "pb-48" } else { "pb-28" }
            ),
            if let Some(alert_props) = favorite_error() {
                Alert { ..alert_props }
            }
            if let Some(alert_props) = playlist_error() {
                Alert { ..alert_props }
            }

            CollectionHero {
                name: "Favorites".to_string(),
                eyebrow: Some("Auto-generated collection".to_string()),
                thumbnails: thumbnails(),
                count: count(),
                accent: "text-error".to_string(),
                button {
                    class: "btn btn-primary rounded-full gap-2 px-6",
                    disabled: items().is_empty(),
                    onclick: play_all,
                    Icon { icon: LdPlay }
                    span { "Play All" }
                }
                button {
                    class: "btn btn-outline rounded-full gap-2 px-5",
                    disabled: items().is_empty(),
                    onclick: shuffle,
                    Icon { icon: LdShuffle }
                    span { "Shuffle" }
                }
            }

            div { class: "mt-8",
                SectionHeader {
                    title: "Tracks".to_string(),
                    badge: Some(count().to_string()),
                    icon: rsx! { Icon { icon: LdListMusic, class: "size-4 text-error" } },
                }
                div { class: "card mt-2 bg-base-100 border border-base-content/10 shadow-sm rounded-2xl overflow-hidden",
                    if favorite_is_loading() {
                        div { class: "flex h-40 items-center justify-center",
                            LoadingSpinner { size: 8 }
                        }
                    } else if items().is_empty() {
                        EmptyState {
                            title: "No favorites yet".to_string(),
                            message: Some(
                                "Tap the heart on any track to save it here.".to_string(),
                            ),
                            icon: rsx! { Icon { icon: LdHeart, class: "size-7" } },
                        }
                    } else {
                        ul { class: "list divide-y divide-base-content/5",
                            for (index, item) in items().iter().enumerate() {
                                MusicRow { item: item.clone(), index }
                            }
                        }
                    }
                }
            }
        }
    }
}
