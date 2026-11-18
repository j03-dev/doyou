use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdDownload, LdListMusic, LdPlay};
use yt::data_api::types::Item;

use crate::components::alert::Alert;
use crate::components::collection_hero::CollectionHero;
use crate::components::empty_state::EmptyState;
use crate::components::loading::LoadingSpinner;
use crate::components::music_row::MusicRow;
use crate::components::navbar::{NavBar, NavBarItem, NavBarPos};
use crate::components::section_header::SectionHeader;
use crate::context::{use_downloads, use_playback};
use crate::repository;
use crate::repository::db::models::Track;

#[component]
pub fn Downloads() -> Element {
    let downloads = use_downloads();
    let playback = use_playback();

    use_effect(move || {
        downloads.fetch_all();
    });

    let items: Memo<Vec<Item>> = use_memo(move || downloads.items());
    let thumbnails = use_memo(move || {
        downloads
            .tracks
            .read()
            .iter()
            .take(4)
            .map(repository::track_thumbnail)
            .collect::<Vec<_>>()
    });
    let count = use_memo(move || downloads.tracks.read().len());

    use_effect(move || {
        playback.set_queue(items());
    });

    let error = downloads.error;
    let is_loading = downloads.is_loading;

    let play_all = move |_: Event<MouseData>| {
        let queued = items();
        if queued.is_empty() {
            return;
        }
        playback.set_queue(queued);
        playback.start(0);
    };

    rsx! {
        NavBar {
            NavBarItem { position: NavBarPos::Start,
                span {}
            }
            NavBarItem { position: NavBarPos::Center,
                p { class: "btn btn-ghost text-xl", "Downloads" }
            }
            NavBarItem { position: NavBarPos::End,
                span {}
            }
        }
        div {
            class: format!(
                "px-4 pt-4 mx-auto w-full max-w-4xl sm:pt-6 {}",
                if playback.playing.read().is_some() { "pb-48" } else { "pb-28" },
            ),
            if let Some(alert_props) = error() {
                Alert { ..alert_props }
            }

            CollectionHero {
                name: "Downloads".to_string(),
                thumbnails: thumbnails(),
                count: count(),
                accent: "text-primary".to_string(),
                button {
                    class: "btn btn-primary rounded-full gap-2 px-6",
                    disabled: items().is_empty(),
                    onclick: play_all,
                    Icon { icon: LdPlay }
                    span { "Play All" }
                }
            }

            div { class: "mt-8",
                SectionHeader {
                    title: "Offline tracks".to_string(),
                    badge: Some(count().to_string()),
                    icon: rsx! {
                        Icon { icon: LdListMusic, class: "size-4 text-primary" }
                    },
                }
                div { class: "card mt-2 bg-base-100 border border-base-content/10 shadow-sm rounded-2xl overflow-hidden",
                    if is_loading() {
                        div { class: "flex h-40 items-center justify-center",
                            LoadingSpinner { size: 8 }
                        }
                    } else if items().is_empty() {
                        EmptyState {
                            title: "No downloads yet".to_string(),
                            message: Some("Use the download button on any YouTube track to save it here.".to_string()),
                            icon: rsx! {
                                Icon { icon: LdDownload, class: "size-7" }
                            },
                        }
                    } else {
                        ul { class: "list divide-y divide-base-content/5",
                            for (index, item) in items().iter().enumerate() {
                                {
                                    let track = downloads.tracks.read().get(index).cloned();
                                    let meta = track.as_ref().and_then(track_meta);
                                    let remove_id = track.map(|track| track.id);
                                    rsx! {
                                        MusicRow {
                                            item: item.clone(),
                                            index,
                                            meta,
                                            on_remove: remove_id.map(|id| EventHandler::new(move |_| {
                                                downloads.remove(&id)
                                            })),
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
}

fn track_meta(track: &Track) -> Option<String> {
    let mut parts = Vec::new();
    if let Some(album) = track.album.as_deref()
        && !album.is_empty()
    {
        parts.push(album.to_string());
    }
    if let Some(duration) = track.duration {
        parts.push(format_duration(duration));
    }
    (!parts.is_empty()).then(|| parts.join(" · "))
}

fn format_duration(seconds: f64) -> String {
    let total = seconds.round().max(0.0) as u64;
    format!("{}:{:02}", total / 60, total % 60)
}
