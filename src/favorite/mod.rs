use dioxus::prelude::*;
use yt::data_api::types::Item;
use yt::data_api::types::Snippet;
use yt::data_api::types::Thumb;
use yt::data_api::types::Thumbnails;
use yt::data_api::types::VideoId;

use crate::common::components::alert_message::AlertMessage;
use crate::common::components::button::IconButton;
use crate::common::components::icons::FavoriteIcon;
use crate::common::components::icons::PauseIcon;
use crate::common::components::icons::PlayIcon;
use crate::common::context::use_playback;
use crate::common::context::{use_alert, use_favorites};
use crate::core::db::models::YoutubeTrack;

#[component]
pub fn FavoriteList() -> Element {
    let favorites = use_favorites();
    let mut alert = use_alert();
    let mut playback = use_playback();

    use_effect(move || {
        favorites.fetch_all();
    });

    use_effect(move || {
        if let Some(err_msg) = favorites.error.read().as_ref() {
            alert.message.set(Some(err_msg.clone()));
        }
    });

    let remove_track = move |track_id: String| {
        favorites.remove(&track_id);
    };

    let play_track = move |index: usize| {
        let items: Vec<Item> = favorites
            .tracks
            .read()
            .iter()
            .map(|t| Item {
                id: VideoId::Literal(t.id.clone()),
                snippet: Snippet {
                    title: t.title.clone(),
                    channel_title: t.channel_name.clone(),
                    thumbnails: Thumbnails {
                        high: Thumb {
                            url: t.thumbnail_url.clone(),
                        },
                    },
                },
            })
            .collect();

        playback.queue.set(items);
        playback.start(index);
    };

    rsx! {
        if let Some(message) = &*alert.message.read() {
            AlertMessage { message: message.clone() }
        }
        div { class: "grid grid-cols-1 md:grid-cols-4 gap-4 p-4",
            for (index, track) in favorites.tracks.read().iter().enumerate() {
                FavoriteCard { track: track.clone(), on_remove: remove_track, index, on_play: play_track }
            }
        }
    }
}

#[component]
pub fn FavoriteCard(
    track: YoutubeTrack,
    on_remove: Callback<String>,
    index: usize,
    on_play: Callback<usize>,
) -> Element {
    let playback = use_playback();
    let track_id = track.id.clone();

    let is_playing_now = use_memo({
        let track_id = track_id.clone();
        move || {
            playback
                .playing
                .read()
                .as_ref()
                .map(|i| i.id.as_string().unwrap())
                == Some(track_id.clone())
                && *playback.is_playing.read()
        }
    });

    let is_loading = use_memo({
        let track_id = track_id.clone();
        move || {
            playback
                .playing
                .read()
                .as_ref()
                .map(|i| i.id.as_string().unwrap())
                == Some(track_id.clone())
                && *playback.is_loading.read()
        }
    });

    rsx! {
        div { class: "card bg-base-200 shadow-md hover:shadow-xl transition-all duration-300 cursor-pointer overflow-hidden",
            if is_playing_now() {
                div { class: "absolute inset-0 bg-base-content/20 z-10" }
            }
            figure { class: "w-full h-40 relative",
                img {
                    src: track.thumbnail_url,
                    class: "w-full h-full px-5 py-5 object-cover",
                    alt: "{track.title}",
                }
                if is_loading() {
                    div { class: "absolute inset-0 flex items-center justify-center bg-base-100/50",
                        span { class: "loading loading-dots loading-lg" }
                    }
                }
            }
            div { class: "card-body p-3",
                div { class: "flex items-start justify-between gap-2",
                    div { class: "flex-1 min-w-0",
                        h2 { class: "card-title text-sm truncate text-base-content",
                            "{track.title}"
                        }
                        p { class: "text-xs text-base-content/70 truncate", "{track.channel_name}" }
                    }
                    div { class: "flex gap-1",
                        IconButton { on_click: move |_| on_remove.call(track_id.clone()),
                            FavoriteIcon { class: "w-5 h-5 fill-error stroke-error" }
                        }
                        IconButton {
                            on_click: move |_| on_play.call(index),
                            if is_playing_now() {
                                PauseIcon { class: "w-5 h-5" }
                            } else {
                                PlayIcon { class: "w-5 h-5" }
                            }
                        }
                    }
                }
            }
        }
    }
}
