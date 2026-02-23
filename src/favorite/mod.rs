use dioxus::prelude::*;

use crate::common::components::alert_message::AlertMessage;
use crate::common::components::button::IconButton;
use crate::common::components::icons::FavoriteIcon;
use crate::common::context::{use_alert, use_favorites};
use crate::core::db;

#[component]
pub fn Favorite() -> Element {
    let mut alert = use_alert();
    let mut favorites = use_favorites();

    use_effect(move || {
        spawn(async move {
            favorites.is_loading.set(true);
            match db::get_all_favorites().await {
                Ok(favs) => favorites.tracks.set(favs),
                Err(e) => {
                    dbg!(e);
                }
            }
            favorites.is_loading.set(false);
        });
    });

    let remove_track = move |track_id: String| {
        spawn(async move {
            match db::remove_from_favorite(&track_id).await {
                Ok(()) => favorites.tracks.write().retain(|t| t.id != track_id),
                Err(e) => alert.message.set(Some(e.to_string())),
            }
        });
    };

    rsx! {
        if let Some(message) = &*alert.message.read() {
            AlertMessage { message: message.clone() }
        }
        div { class: "grid grid-cols-1 md:grid-cols-4 gap-6 p-4 justify-items-center",
            for track in favorites.tracks.read().iter() {
                {
                    let track_id = track.id.clone();
                    let thumbnail = track.thumbnail_url.clone();
                    rsx! {
                        div { class: "card card-sm bg-base-100 w-full max-w-sm shadow-sm",
                            figure { class: "px-5 pt-5",
                                img { src: thumbnail, class: "rounded-xl size-full" }
                            }
                            div { class: "card-body items-start text-left",
                                div {
                                    h2 { class: "card-title text-sm", "{track.title}" }
                                    p { class: "text-xs opacity-70", "{track.channel_name}" }
                                }
                                IconButton {
                                    on_click: move |_| remove_track(track_id.clone()),
                                    FavoriteIcon { class: "fill-red-500 stroke-current-500" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
