use dioxus::prelude::*;

use crate::common::components::alert_message::AlertMessage;
use crate::common::components::button::Button;
use crate::common::components::button::IconButton;
use crate::common::components::icons::FavoriteIcon;
use crate::common::components::icons::PlayIcon;
use crate::common::context::{use_alert, use_favorites};
use crate::core::db::models::YoutubeTrack;

#[component]
pub fn FavoriteList() -> Element {
    let favorites = use_favorites();
    let mut alert = use_alert();

    use_effect(move || {
        favorites.fetch_all();
    });

    use_effect(move || {
        if let Some(err_msg) = favorites.error.read().as_ref() {
            alert.message.set(Some(err_msg.clone()));
        }
    });

    let remove_track = move |track_id: String| {
        let favorites = use_favorites();
        favorites.remove(&track_id);
    };

    rsx! {
        if let Some(message) = &*alert.message.read() {
            AlertMessage { message: message.clone() }
        }
        div { class: "grid grid-cols-1 md:grid-cols-4 gap-4 p-4",
            for track in favorites.tracks.read().iter() {
                FavoriteCard { track: track.clone(), on_remove: remove_track }
            }
        }
    }
}

#[component]
pub fn FavoriteCard(track: YoutubeTrack, on_remove: Callback<String>) -> Element {
    let track_id = track.id.clone();

    rsx! {
        div { class: "card bg-base-200 shadow-md hover:shadow-xl transition-all duration-300 cursor-pointer overflow-hidden",
            figure { class: "w-full h-40",
                img {
                    src: track.thumbnail_url,
                    class: "w-full h-full px-5 py-5 object-cover",
                    alt: "{track.title}",
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
                        Button {
                            on_click: move |_| {},
                            class: "btn-primary rounded rounded-xl",
                            "play"
                            PlayIcon { class: "w-5 h-5" }
                        }
                    }
                }
            }
        }
    }
}
