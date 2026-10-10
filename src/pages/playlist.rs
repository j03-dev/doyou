use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
    LdArrowLeft, LdEllipsis, LdListMusic, LdMusic, LdPlay, LdShuffle, LdTrash2,
};
use yt::data_api::types::Item;

use crate::components::alert::Alert;
use crate::components::collection_hero::CollectionHero;
use crate::components::empty_state::EmptyState;
use crate::components::loading::LoadingSpinner;
use crate::components::music_row::MusicRow;
use crate::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::components::section_header::SectionHeader;
use crate::context::{use_playback, use_playlists};
use crate::repository::tracks_to_items;
use crate::route::Route;

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

    let shuffle = move |_: Event<MouseData>| {
        playback.start_shuffled(items());
    };

    rsx! {
        NavBar {
            NavBarItem { position: NavBarPos::Start,
                button {
                    class: "btn btn-ghost btn-circle",
                    "aria-label": "Back",
                    onclick: move |_| nav.go_back(),
                    Icon { icon: LdArrowLeft }
                }
            }
            NavBarItem { position: NavBarPos::Center,
                p { class: "btn btn-ghost text-xl", "Playlist" }
            }
            NavBarItem { position: NavBarPos::End,
                div { class: "dropdown dropdown-end",
                    div {
                        tabindex: 0,
                        role: "button",
                        class: "btn btn-ghost btn-circle",
                        "aria-label": "Playlist options",
                        Icon { icon: LdEllipsis }
                    }
                    ul {
                        tabindex: 0,
                        class: "dropdown-content menu bg-base-200 rounded-box z-10 w-48 p-2 shadow-lg",
                        li {
                            button {
                                class: "text-error flex items-center gap-2",
                                onclick: move |_| {
                                    spawn(async move {
                                        let _ = document::eval("delete_playlist_modal.showModal()").await;
                                    });
                                },
                                Icon { icon: LdTrash2, class: "size-4" }
                                "Delete playlist"
                            }
                        }
                    }
                }
            }
        }
        div { class: format!(
                "px-4 pt-4 mx-auto w-full max-w-4xl sm:pt-6 {}",
                if playback.playing.read().is_some() { "pb-48" } else { "pb-28" }
            ),
            if let Some(alert_props) = error() {
                Alert { ..alert_props }
            }
            if is_loading() {
                div { class: "flex h-40 justify-center items-center",
                    LoadingSpinner { size: 8 }
                }
            } else if let Some(entry) = detail() {
                CollectionHero {
                    name: entry.playlist.name.clone(),
                    eyebrow: Some("Playlist".to_string()),
                    thumbnails: entry.thumbnails(),
                    count: entry.tracks.len(),
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
                        badge: Some(entry.tracks.len().to_string()),
                        icon: rsx! { Icon { icon: LdListMusic, class: "size-4 text-primary" } },
                    }
                    div { class: "card mt-2 bg-base-100 border border-base-content/10 shadow-sm rounded-2xl overflow-hidden",
                        if items().is_empty() {
                            EmptyState {
                                title: "No songs yet".to_string(),
                                message: Some(
                                    "Add tracks from the library or search results.".to_string(),
                                ),
                                icon: rsx! { Icon { icon: LdMusic, class: "size-7" } },
                            }
                        } else {
                            ul { class: "list divide-y divide-base-content/5",
                                for (index, item) in items().iter().enumerate() {
                                    {
                                        let track_id = item.id.as_string().unwrap_or_default();
                                        rsx! {
                                            MusicRow {
                                                item: item.clone(),
                                                index,
                                                on_remove: move |_| {
                                                    playlists.remove_track(id, track_id.clone());
                                                },
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        dialog { id: "delete_playlist_modal", class: "modal",
            div { class: "modal-box w-96 max-w-[calc(100vw-2rem)]",
                h3 { class: "font-bold text-lg text-error", "Delete Playlist" }
                p { class: "py-4 text-sm text-base-content/70",
                    "Are you sure you want to delete this playlist? This action cannot be undone."
                }
                div { class: "modal-action flex justify-end gap-2",
                    form { method: "dialog",
                        button { class: "btn", "Cancel" }
                    }
                    button {
                        class: "btn btn-error",
                        onclick: move |_| {
                            playlists.remove(id);
                            nav.replace(Route::Home {});
                        },
                        "Delete"
                    }
                }
            }
            form { method: "dialog", class: "modal-backdrop",
                button { "close" }
            }
        }
    }
}
