use dioxus::prelude::*;

use crate::common::components::alert_message::AlertMessage;
use crate::common::components::button::IconButton;
use crate::common::components::icons::FavoriteIcon;
use crate::common::context::{use_alert, use_favorites};
use crate::core::db;

#[component]
pub fn Favorite() -> Element {
    let alert = use_alert();
    let alert_message = alert.message;

    let favorites = use_favorites();
    let fav_tracks = favorites.tracks;
    let fav_loading = favorites.is_loading;

    use_effect(move || {
        let mut tracks = fav_tracks;
        let mut loading = fav_loading;

        spawn(async move {
            loading.set(true);
            match db::get_all_favorites().await {
                Ok(favs) => tracks.set(favs),
                Err(e) => {
                    dbg!(e);
                }
            }
            loading.set(false);
        });
    });

    let remove_track = move |track_id: String| {
        let mut tracks = fav_tracks;
        let mut msg = alert_message;

        spawn(async move {
            match db::remove_from_favorite(&track_id).await {
                Ok(()) => {
                    tracks.write().retain(|t| t.id != track_id);
                }
                Err(e) => msg.set(Some(e.to_string())),
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
                    rsx! {
                        div { class: "card card-sm bg-base-100 w-full max-w-sm shadow-sm",
                            figure { class: "px-5 pt-5",
                                img { src: track.thumbnail_url.clone(), class: "rounded-xl size-full" }
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
